use crate::model::{ModelBuilderContext, build_model_pipeline};
use crate::schedule::{CourseCode, Schedule};
use anyhow::{Result, anyhow};
use cp_sat::builder::{CpModelBuilder, IntVar, LinearExpr};
use cp_sat::proto::CpSolverStatus;

/// Returns Some(Vec<Vec<(CourseCode, i64)>>) if a feasible schedule is found, else None.
pub fn two_stage_lex_schedule(sched: &mut Schedule, max_credits_per_semester: i64) -> Result<()> {
    let mut params = cp_sat::proto::SatParameters::default();
    params.log_search_progress = Some(true);
    params.num_search_workers = Some(8);
    // Fix solver seed for reproducibility and easier debugging
    params.random_seed = Some(123456);
    // --- Transform schedule: add incoming as semester 0 (always present, even if empty) ---
    // Always set incoming courses before building model context for both stages
    let mut sched_for_model = sched.clone();
    sched_for_model.incoming = sched.incoming.clone();
    let mut all_semesters = vec![sched_for_model.incoming.clone()];
    all_semesters.extend(sched.courses.clone());
    sched_for_model.courses = all_semesters;
    // Stage 1: minimize total credits
    let mut ctx = ModelBuilderContext::new(&sched_for_model, max_credits_per_semester);
    let (mut model, vars, flat_courses) = build_model_pipeline(&mut ctx);
    // Determine number of semesters (includes incoming semester 0)
    let num_semesters = sched_for_model.courses.len();
    // Diagnostic: print context info before solving stage 1
    println!("[DIAG] Stage 1 incoming_codes: {:?}", ctx.incoming_codes);
    println!(
        "[DIAG] Stage 1 num_semesters: {} flat_courses: {}",
        num_semesters,
        flat_courses.len()
    );
    let first_sched_semester = 1; // semester 0 is incoming only
    // Only sum credits for semesters 1..N (ignore semester 0)
    let mut total_credits_sched = cp_sat::builder::LinearExpr::from(0);
    for s in first_sched_semester..num_semesters {
        for i in 0..flat_courses.len() {
            let credits = flat_courses[i].1;
            total_credits_sched = total_credits_sched + (credits, vars[i][s].clone());
        }
    }
    model.minimize(total_credits_sched.clone());
    let response = model.solve_with_parameters(&params);
    println!("[DIAG] Stage 1 solver status: {:?}", response.status());

    // Compute min_credits as the sum of all scheduled (assigned + prereq) course credits in the solution
    let min_credits = match response.status() {
        CpSolverStatus::Optimal | CpSolverStatus::Feasible => {
            let mut total = 0;
            for (i, (_course, credits)) in flat_courses.iter().enumerate() {
                for s in first_sched_semester..num_semesters {
                    if vars[i][s].solution_value(&response) {
                        total += credits;
                    }
                }
            }
            total
        }
        _ => {
            // No feasible solution
            return Err(anyhow!(
                "No feasible solution found in single-stage scheduling"
            ));
        }
    };

    // Stage 2: minimize spread, subject to min total credits
    let mut ctx2 = ModelBuilderContext::new(&sched_for_model, max_credits_per_semester);
    ctx2.set_min_credits(min_credits);
    let (mut model2, vars2, flat_courses2) = build_model_pipeline(&mut ctx2);
    // Diagnostic: print incoming codes in stage two
    println!("[DIAG] Stage 2 incoming_codes: {:?}", ctx2.incoming_codes);
    // Compute mean load (rounded down), EXCLUDING semester 0 (incoming)
    let num_sched_semesters = num_semesters as i64 - 1;
    let mean_load = if num_sched_semesters > 0 {
        min_credits / num_sched_semesters
    } else {
        0
    };

    // For each semester, create an IntVar for the semester's total credits
    let mut semester_credit_vars = Vec::new();
    for s in first_sched_semester..num_semesters {
        let mut expr = LinearExpr::from(0);
        for i in 0..flat_courses2.len() {
            let var = vars2[i][s].clone();
            let coeff = flat_courses2[i].1;
            // Use a single scaled addition (supports negative coeffs) instead of repeated adds/subtracts
            expr = expr + (coeff, var.clone());
        }
        // Domain: [0, max_credits_per_semester * flat_courses2.len() as i64]
        let domain = vec![(0, max_credits_per_semester * flat_courses2.len() as i64)];
        let var = model2.new_int_var(domain.clone());
        model2.add_eq(var.clone(), expr);
        semester_credit_vars.push(var);
    }

    // For each semester, create an IntVar for the absolute deviation from mean
    let mut abs_deviation_vars = Vec::new();
    for credit_var in semester_credit_vars.iter() {
        let diff_domain = vec![(
            -max_credits_per_semester * flat_courses2.len() as i64,
            max_credits_per_semester * flat_courses2.len() as i64,
        )];
        let diff = model2.new_int_var(diff_domain);
        // diff = semester_credits - mean_load
        model2.add_eq(
            diff.clone(),
            LinearExpr::from(credit_var.clone()) - mean_load,
        );
        let abs_domain = vec![(0, max_credits_per_semester * flat_courses2.len() as i64)];
        let abs_diff = model2.new_int_var(abs_domain);
        // abs_diff >= diff
        model2.add_ge(abs_diff.clone(), LinearExpr::from(diff.clone()));
        // abs_diff >= -diff (negate by repeated subtraction)
        let mut neg_diff_expr = LinearExpr::from(0);
        for _ in 0..1 {
            // -1 * diff
            neg_diff_expr = neg_diff_expr - LinearExpr::from(diff.clone());
        }
        model2.add_ge(abs_diff.clone(), neg_diff_expr);
        abs_deviation_vars.push(abs_diff);
    }
    // Minimize the sum of absolute deviations (primary objective)
    let mut spread_penalty = LinearExpr::from(0);
    for v in &abs_deviation_vars {
        spread_penalty = spread_penalty + LinearExpr::from(v.clone());
    }

    // --- Ordering objective: penalize semesters where mean course code does not increase ---
    let mut sum_codes = Vec::new();
    let mut count_courses = Vec::new();
    for s in first_sched_semester..num_semesters {
        let mut sum = LinearExpr::from(0);
        let mut count = LinearExpr::from(0);
        for i in 0..flat_courses2.len() {
            let code = &flat_courses2[i].0.code;
            let val = match &code.code {
                crate::schedule::CourseCodeSuffix::Number(n)
                | crate::schedule::CourseCodeSuffix::Unique(n) => *n as i64,
                crate::schedule::CourseCodeSuffix::Special(x) => {
                    if x.as_str() == "COMP" {
                        1000000 // Assign a high value for COMP courses
                    } else {
                        0 // Other special codes treated as 0
                    }
                }
            };
            sum = sum + (val, vars2[i][s].clone());
            count = count + LinearExpr::from(vars2[i][s].clone());
        }
        sum_codes.push(sum);
        count_courses.push(count);
    }
    let mut order_penalty = LinearExpr::from(0);
    for s in 0..(sum_codes.len() - 1) {
        // Penalize if sum in s > sum in s+1 (approximate ascending order)
        let diff = sum_codes[s].clone() - sum_codes[s + 1].clone();
        // Only penalize positive differences
        let diff_var = model2.new_int_var(vec![(0, 1000000)]);
        model2.add_ge(diff_var.clone(), diff);
        order_penalty = order_penalty + diff_var;
    }
    // Add mini-objective to main objective (small weight)
    let mut weighted_spread = LinearExpr::from(0);
    for _ in 0..50 {
        weighted_spread = weighted_spread + spread_penalty.clone();
    }
    let total_objective = weighted_spread + order_penalty;
    model2.minimize(total_objective);

    let response2 = model2.solve_with_parameters(&params);
    match response2.status() {
        CpSolverStatus::Optimal | CpSolverStatus::Feasible => {
            // Build the schedule output: Vec<Vec<(CourseCode, i64)>>
            let mut result = vec![vec![]; num_semesters];
            for (i, (course, credits)) in flat_courses2.iter().enumerate() {
                for s in 0..num_semesters {
                    if vars2[i][s].solution_value(&response2) {
                        result[s].push((course.code.clone(), *credits));
                    }
                }
            }
            // Diagnostic: print all courses scheduled in semester 0 (incoming)
            println!("[DIAG] Scheduled in semester 0 (incoming):");
            for (code, credits) in &result[0] {
                println!("[DIAG]   {} ({} credits)", code, credits);
                if !sched.incoming.contains(code) {
                    println!(
                        "[WARNING] Non-incoming course scheduled in semester 0: {} ({} credits)",
                        code, credits
                    );
                }
            }
            // Strictly separate incoming (semester 0) from planned semesters (1..N)
            // Only incoming courses should be present in semester 0
            let filtered_incoming_codes: Vec<CourseCode> = result[0]
                .iter()
                .filter(|(code, _)| sched.incoming.contains(code))
                .map(|(code, _)| code.clone())
                .collect();
            sched.incoming = filtered_incoming_codes;
            // Only planned semesters (1..N) go into sched.courses
            sched.courses = result
                .iter()
                .skip(first_sched_semester)
                .map(|sem| sem.iter().map(|(code, _)| code.clone()).collect())
                .collect();
            Ok(())
        }
        _ => Err(anyhow!(
            "No feasible solution found in two-stage scheduling"
        )),
    }
}
