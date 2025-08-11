use cp_sat::builder::{CpModelBuilder, LinearExpr};
use cp_sat::proto::CpSolverStatus;
use std::collections::HashMap;


#[path = "../../src/schedule.rs"]
mod schedule;

#[path = "../../src/prereqs.rs"]
mod prereqs;

#[path = "../../src/geneds.rs"]
mod geneds;

use prereqs::CourseReq;
use schedule::CourseCode;

#[derive(Clone)]
struct Course<'a> {
    code: CourseCode,
    credits: i64,
    required: bool,
    geneds: Vec<&'a str>,
    elective_group: Option<&'a str>,
    prereqs: CourseReq,
}

/// Build the model and return (model, vars, total_credits LinearExpr)
fn build_model<'a>(courses: &'a [Course<'a>], num_semesters: usize, max_credits_per_semester: i64, min_credits: Option<i64>) -> (CpModelBuilder, Vec<Vec<cp_sat::builder::BoolVar>>, LinearExpr) {
    let mut model = CpModelBuilder::default();
    let mut vars = Vec::new();
    for i in 0..courses.len() {
        let mut sem_vars = Vec::new();
        for s in 0..num_semesters {
            let v = model.new_bool_var_with_name(format!("c_{}_{}", i, s));
            sem_vars.push(v);
        }
        vars.push(sem_vars);
    }
    // Required courses exactly once
    for (i, c) in courses.iter().enumerate() {
        if c.required {
            model.add_exactly_one(vars[i].iter().copied());
        }
    }
    // Optional courses at most once
    for (i, c) in courses.iter().enumerate() {
        if !c.required {
            model.add_at_most_one(vars[i].iter().copied());
        }
    }
    // Prerequisite constraints
    let idx_map: HashMap<_, _> = courses.iter().enumerate().map(|(i, c)| (c.code.clone(), i)).collect();
    // Recursively add prerequisite constraints for each course and semester
    fn add_prereq_constraints(
        model: &mut CpModelBuilder,
        vars: &Vec<Vec<cp_sat::builder::BoolVar>>,
        idx_map: &HashMap<CourseCode, usize>,
        courses: &[Course],
        course_idx: usize,
        req: &CourseReq,
        num_semesters: usize,
    ) {
        use prereqs::CourseReq::*;
        match req {
            And(reqs) => {
                for r in reqs {
                    add_prereq_constraints(model, vars, idx_map, courses, course_idx, r, num_semesters);
                }
            }
            Or(reqs) => {
                // For each semester, if this course is taken, at least one of the OR prereqs must be satisfied
                for s in 0..num_semesters {
                    let cur = vars[course_idx][s];
                    let mut or_exprs = Vec::new();
                    for r in reqs {
                        // For each sub-req, create a bool var indicating if it is satisfied by semester s
                        let or_var = model.new_bool_var();
                        // Recursively add constraints for this sub-req, but only if or_var is true
                        // We use indicator constraints: if or_var == 1, then sub-req must be satisfied
                        // For PreCourse/CoCourse, this is just a linear constraint
                        match r {
                            PreCourse(code) => {
                                if let Some(&pre_idx) = idx_map.get(code) {
                                    if s == 0 {
                                        // If s == 0, can't satisfy PreCourse, so or_var must be false if cur is true
                                        // Instead, just don't add or_var to or_exprs in this case
                                    } else {
                                        let earlier_vars: Vec<_> = vars[pre_idx][..s].iter().copied().collect();
                                        if !earlier_vars.is_empty() {
                                            let sum_earlier: LinearExpr = earlier_vars.into_iter().collect();
                                            // If cur is taken, require sum_earlier >= 1
                                            // or_var = 1 <=> sum_earlier >= 1
                                            // We approximate: sum_earlier >= or_var
                                            model.add_linear_constraint(sum_earlier - or_var, [(0, i64::MAX)]);
                                            or_exprs.push(or_var);
                                        }
                                    }
                                }
                            }
                            CoCourse(code) => {
                                if let Some(&co_idx) = idx_map.get(code) {
                                    let upto_vars: Vec<_> = vars[co_idx][..=s].iter().copied().collect();
                                    if !upto_vars.is_empty() {
                                        let sum_upto: LinearExpr = upto_vars.into_iter().collect();
                                        model.add_linear_constraint(sum_upto - or_var, [(0, i64::MAX)]);
                                        or_exprs.push(or_var);
                                    }
                                }
                            }
                            And(_) | Or(_) => {
                                // Recursively add for sub-reqs
                                add_prereq_constraints(model, vars, idx_map, courses, course_idx, r, num_semesters);
                                // For OR, we just add the or_var to the or_exprs
                            }
                            _ => unimplemented!("Only PreCourse, CoCourse, And, Or supported"),
                        }
                        or_exprs.push(or_var);
                    }
                    // If cur is taken, at least one or_var must be true
                    if !or_exprs.is_empty() {
                        let sum_or: LinearExpr = or_exprs.iter().copied().collect();
                        model.add_linear_constraint(sum_or - cur, [(0, i64::MAX)]);
                    }
                }
            }
            PreCourse(code) => {
                if let Some(&pre_idx) = idx_map.get(code) {
                    for s in 0..num_semesters {
                        let cur = vars[course_idx][s];
                        if s == 0 {
                            model.add_eq(cur, 0);
                        } else {
                            let earlier_vars: Vec<_> = vars[pre_idx][..s].iter().copied().collect();
                            if !earlier_vars.is_empty() {
                                let sum_earlier: LinearExpr = earlier_vars.into_iter().collect();
                                model.add_linear_constraint(sum_earlier - cur, [(0, i64::MAX)]);
                            } else {
                                model.add_eq(cur, 0);
                            }
                        }
                    }
                } else {
                    // If prereq course not found, can't take this course
                    for s in 0..num_semesters {
                        model.add_eq(vars[course_idx][s], 0);
                    }
                }
            }
            CoCourse(code) => {
                if let Some(&co_idx) = idx_map.get(code) {
                    for s in 0..num_semesters {
                        let cur = vars[course_idx][s];
                        let upto_vars: Vec<_> = vars[co_idx][..=s].iter().copied().collect();
                        if !upto_vars.is_empty() {
                            let sum_upto: LinearExpr = upto_vars.into_iter().collect();
                            model.add_linear_constraint(sum_upto - cur, [(0, i64::MAX)]);
                        } else {
                            model.add_eq(cur, 0);
                        }
                    }
                } else {
                    for s in 0..num_semesters {
                        model.add_eq(vars[course_idx][s], 0);
                    }
                }
            }
            _ => unimplemented!("Only PreCourse, CoCourse, And, Or supported"),
        }
    }
    for (i, c) in courses.iter().enumerate() {
        add_prereq_constraints(&mut model, &vars, &idx_map, courses, i, &c.prereqs, num_semesters);
    }
    // Gen-ed requirements
    let gened_reqs = vec![("WRI", 1), ("HUM", 1), ("SCI", 1)];
    for &(g, req) in &gened_reqs {
        let mut all_vars = Vec::new();
        for (i, c) in courses.iter().enumerate() {
            if c.geneds.contains(&g) {
                all_vars.extend(vars[i].iter().copied());
            }
        }
        if !all_vars.is_empty() {
            let sum: LinearExpr = all_vars.into_iter().collect();
            model.add_ge(sum, req);
        }
    }
    // Elective group requirements
    let elective_reqs = vec![("ELEC_A", 1)];
    for &(eg, req) in &elective_reqs {
        let mut all_vars = Vec::new();
        for (i, c) in courses.iter().enumerate() {
            if c.elective_group == Some(eg) {
                all_vars.extend(vars[i].iter().copied());
            }
        }
        if !all_vars.is_empty() {
            let sum: LinearExpr = all_vars.into_iter().collect();
            model.add_ge(sum, req);
        }
    }
    // Semester credit limits
    for s in 0..num_semesters {
        let weighted_terms: Vec<(i64, _)> = courses.iter().enumerate()
            .map(|(i, c)| (c.credits, vars[i][s]))
            .collect();
        let weighted_sum: LinearExpr = weighted_terms.into_iter().collect();
        model.add_le(weighted_sum, max_credits_per_semester);
    }
    // Objective: minimize total credits
    let mut obj_terms = Vec::new();
    for (i, c) in courses.iter().enumerate() {
        for s in 0..num_semesters {
            obj_terms.push((c.credits, vars[i][s]));
        }
    }
    let total_credits: LinearExpr = obj_terms.into_iter().collect();
    if let Some(min_credits) = min_credits {
        model.add_eq(total_credits.clone(), min_credits);
    }
    (model, vars, total_credits)
}

fn main() {
    use prereqs::CourseReq::*;
    use schedule::{CourseCode, CourseCodeSuffix};
    let courses = vec![
        Course {
            code: CourseCode { stem: "MATH".to_string(), code: CourseCodeSuffix::from(101) },
            credits: 4,
            required: true,
            geneds: vec![],
            elective_group: None,
            prereqs: NotRequired,
        },
        Course {
            code: CourseCode { stem: "MATH".to_string(), code: CourseCodeSuffix::from(102) },
            credits: 4,
            required: true,
            geneds: vec![],
            elective_group: None,
            prereqs: PreCourse(CourseCode { stem: "MATH".to_string(), code: CourseCodeSuffix::from(101) }),
        },
        Course {
            code: CourseCode { stem: "CS".to_string(), code: CourseCodeSuffix::from(101) },
            credits: 3,
            required: true,
            geneds: vec![],
            elective_group: None,
            prereqs: NotRequired,
        },
        Course {
            code: CourseCode { stem: "CS".to_string(), code: CourseCodeSuffix::from(201) },
            credits: 3,
            required: true,
            geneds: vec![],
            elective_group: None,
            prereqs: PreCourse(CourseCode { stem: "CS".to_string(), code: CourseCodeSuffix::from(101) }),
        },
        Course {
            code: CourseCode { stem: "ENG".to_string(), code: CourseCodeSuffix::from(1) },
            credits: 3,
            required: false,
            geneds: vec!["WRI"],
            elective_group: None,
            prereqs: NotRequired,
        },
        Course {
            code: CourseCode { stem: "PHIL".to_string(), code: CourseCodeSuffix::from(1) },
            credits: 3,
            required: false,
            geneds: vec!["HUM"],
            elective_group: None,
            prereqs: NotRequired,
        },
        Course {
            code: CourseCode { stem: "BIO".to_string(), code: CourseCodeSuffix::from(1) },
            credits: 4,
            required: false,
            geneds: vec!["SCI"],
            elective_group: None,
            prereqs: NotRequired,
        },
        Course {
            code: CourseCode { stem: "ART".to_string(), code: CourseCodeSuffix::from(1) },
            credits: 3,
            required: false,
            geneds: vec!["ART"],
            elective_group: None,
            prereqs: NotRequired,
        },
        Course {
            code: CourseCode { stem: "ELEC_A".to_string(), code: CourseCodeSuffix::from(1) },
            credits: 2,
            required: false,
            geneds: vec![],
            elective_group: Some("ELEC_A"),
            prereqs: NotRequired,
        },
        Course {
            code: CourseCode { stem: "ELEC_A".to_string(), code: CourseCodeSuffix::from(2) },
            credits: 3,
            required: false,
            geneds: vec![],
            elective_group: Some("ELEC_A"),
            prereqs: NotRequired,
        },
        Course {
            code: CourseCode { stem: "CS".to_string(), code: CourseCodeSuffix::from(301) },
            credits: 3,
            required: false,
            geneds: vec![],
            elective_group: None,
            prereqs: PreCourse(CourseCode { stem: "CS".to_string(), code: CourseCodeSuffix::from(201) }),
        },
    ];
    let num_semesters = 8;
    let max_credits_per_semester = 18;

    // Stage 1: minimize total credits
    let (mut model, vars, total_credits) = build_model(&courses, num_semesters, max_credits_per_semester, None);
    model.minimize(total_credits.clone());
    let response = model.solve();
    let mut min_credits = None;
    if let CpSolverStatus::Optimal | CpSolverStatus::Feasible = response.status() {
        min_credits = Some(response.objective_value as i64);
    }

    // Add a constant to toggle the secondary objective
    const SPREAD_SEMESTERS: bool = true;

    // Stage 2: minimize spread, subject to min total credits
    if SPREAD_SEMESTERS {
        if let Some(min_credits) = min_credits {
            let (mut model2, vars2, total_credits2) = build_model(&courses, num_semesters, max_credits_per_semester, Some(min_credits));
            // Compute mean load (rounded down)
            let total_credits_int: i64 = courses.iter().map(|c| c.credits).sum();
            let mean_load = total_credits_int / num_semesters as i64;
            let mut abs_deviation_vars = Vec::new();
            for s in 0..num_semesters {
                let semester_credits: LinearExpr = courses.iter().enumerate()
                    .map(|(i, c)| (c.credits, vars2[i][s]))
                    .collect();
                let deviation = semester_credits.clone() - mean_load;
                let abs_dev = model2.new_int_var([(0, 1000)]);
                model2.add_ge(abs_dev, deviation.clone());
                model2.add_ge(abs_dev, -deviation);
                abs_deviation_vars.push(abs_dev);
            }
            let spread_penalty: LinearExpr = abs_deviation_vars.iter().copied().collect();
            model2.minimize(spread_penalty);
            let response2 = model2.solve();
            match response2.status() {
                CpSolverStatus::Optimal | CpSolverStatus::Feasible => {
                    println!("Schedule found (lexicographic):");
                    for s in 0..num_semesters {
                        println!("Semester {}", s + 1);
                        let mut sem_credits = 0;
                        for (i, c) in courses.iter().enumerate() {
                            if vars2[i][s].solution_value(&response2) {
                                println!("  {} ({} credits)", c.code, c.credits);
                                sem_credits += c.credits;
                            }
                        }
                        println!("  Credits: {}", sem_credits);
                    }
                    println!("Total credits: {}", min_credits);
                }
                _ => {
                    println!("No feasible solution found in stage 2. Status: {:?}", response2.status());
                }
            }
            return;
        }
    }
    // Solve and report
    match response.status() {
        CpSolverStatus::Optimal | CpSolverStatus::Feasible => {
            println!("Schedule found:");
            for s in 0..num_semesters {
                println!("Semester {}", s + 1);
                let mut sem_credits = 0;
                for (i, c) in courses.iter().enumerate() {
                    if vars[i][s].solution_value(&response) {
                        println!("  {} ({} credits)", c.code, c.credits);
                        sem_credits += c.credits;
                    }
                }
                println!("  Credits: {}", sem_credits);
            }
            println!("Total credits: {}", response.objective_value as i64);
        }
        _ => {
            println!("No feasible solution found. Status: {:?}", response.status());
        }
    }
}
