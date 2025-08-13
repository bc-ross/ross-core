use crate::model::{ModelBuilderContext, build_model_pipeline};
use crate::schedule::{CourseCode, Schedule};
use cp_sat::builder::{CpModelBuilder, IntVar, LinearExpr};
use cp_sat::proto::CpSolverStatus;
use anyhow::{anyhow, Result};

/// Returns Some(Vec<Vec<(CourseCode, i64)>>) if a feasible schedule is found, else None.
pub fn two_stage_lex_schedule(
    sched: &mut Schedule,
    max_credits_per_semester: i64,
) -> Result<()> {
    let mut params = cp_sat::proto::SatParameters::default();
    params.log_search_progress = Some(true);
    dbg!(params.num_search_workers);
    // Stage 1: minimize total credits
    let mut ctx = ModelBuilderContext::new(sched, max_credits_per_semester);
    let (mut model, vars, flat_courses) = build_model_pipeline(&mut ctx);
    let num_semesters = sched.courses.len();
    let total_credits = ctx.total_credits_expr(&vars, &flat_courses);
    model.minimize(total_credits.clone());
    let response = model.solve();

    // Compute min_credits as the sum of all scheduled (assigned + prereq) course credits in the solution
    let min_credits = match response.status() {
        CpSolverStatus::Optimal | CpSolverStatus::Feasible => {
            let mut total = 0;
            for (i, (_course, credits)) in flat_courses.iter().enumerate() {
                for s in 0..num_semesters {
                    if vars[i][s].solution_value(&response) {
                        total += credits;
                    }
                }
            }
            total
        }
        _ => {
            // No feasible solution
            return Err(anyhow!("No feasible solution found in single-stage scheduling"));
        }
    };

    // Stage 2: minimize spread, subject to min total credits
    let mut ctx2 = ModelBuilderContext::new(sched, max_credits_per_semester);
    ctx2.set_min_credits(min_credits);
    let (mut model2, vars2, flat_courses2) = build_model_pipeline(&mut ctx2);
    // Compute mean load (rounded down)
    let mean_load = min_credits / num_semesters as i64;

    // For each semester, create an IntVar for the semester's total credits
    let mut semester_credit_vars = Vec::new();
    for s in 0..num_semesters {
        let mut expr = LinearExpr::from(0);
        for i in 0..flat_courses2.len() {
            let var = vars2[i][s].clone();
            let coeff = flat_courses2[i].1;
            if coeff > 0 {
                for _ in 0..coeff {
                    expr = expr + LinearExpr::from(var.clone());
                }
            } else if coeff < 0 {
                for _ in 0..(-coeff) {
                    expr = expr - LinearExpr::from(var.clone());
                }
            }
        }
        // Domain: [0, max_credits_per_semester * flat_courses2.len() as i64]
        let domain = vec![(0, max_credits_per_semester * flat_courses2.len() as i64)];
        let var = model2.new_int_var(domain.clone());
        model2.add_eq(var.clone(), expr);
        semester_credit_vars.push(var);
    }

    // For each semester, create an IntVar for the absolute deviation from mean
    let mut abs_deviation_vars = Vec::new();
    for (_s, credit_var) in semester_credit_vars.iter().enumerate() {
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

    // // --- Mini-objective: penalize out-of-order course pairs in each semester ---
    // // Helper to get a numeric value for ordering
    // fn course_order_value(code: &crate::schedule::CourseCode) -> i64 {
    //     use crate::schedule::CourseCodeSuffix::*;
    //     match &code.code {
    //         Number(n) | Unique(n) => *n as i64,
    //         Special(s) if s == "COMP" => i64::MAX, // treat as infinity
    //         Special(_) => -1, // ignore in ordering
    //     }
    // }
    // let mut order_penalty = LinearExpr::from(0);
    // for s in 0..num_semesters {
    //     // For all pairs (i, j) with i < j in flat_courses2
    //     for i in 0..flat_courses2.len() {
    //         for j in (i+1)..flat_courses2.len() {
    //             let code_i = &flat_courses2[i].0.code;
    //             let code_j = &flat_courses2[j].0.code;
    //             let val_i = course_order_value(code_i);
    //             let val_j = course_order_value(code_j);
    //             // Only penalize if both are orderable (not -1)
    //             if val_i >= 0 && val_j >= 0 && val_i > val_j {
    //                 // If both scheduled in semester s, add penalty
    //                 let var_i = vars2[i][s].clone();
    //                 let var_j = vars2[j][s].clone();
    //                 // penalty_var = 1 if both scheduled
    //                 let penalty_var = model2.new_bool_var();
    //                 model2.add_le(penalty_var.clone(), var_i.clone());
    //                 model2.add_le(penalty_var.clone(), var_j.clone());
    //                 model2.add_ge(
    //                     penalty_var.clone(),
    //                     LinearExpr::from(var_i.clone()) + LinearExpr::from(var_j.clone()) - LinearExpr::from(1),
    //                 );
    //                 order_penalty = order_penalty + penalty_var;
    //             }
    //         }
    //     }
    // }
    // // Add mini-objective to main objective (small weight)
    // let mut weighted_spread = LinearExpr::from(0);
    // for _ in 0..1000 {
    //     weighted_spread = weighted_spread + spread_penalty.clone();
    // }
    // let total_objective = weighted_spread + order_penalty;
    model2.minimize(spread_penalty);

    let response2 = model2.solve();
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
            // Overwrite sched.courses with the new schedule (just the codes)
            sched.courses = result
                .iter()
                .map(|sem| sem.iter().map(|(code, _)| code.clone()).collect())
                .collect();
            Ok(())
        }
        _ => {
            Err(anyhow!("No feasible solution found in two-stage scheduling"))
        }
    }
}
