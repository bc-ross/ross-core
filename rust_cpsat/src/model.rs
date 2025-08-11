//! Model building and constraint logic for the course scheduling solver.
use crate::schedule::{CourseCode, Catalog};
use crate::prereqs::CourseReq;
use cp_sat::builder::{CpModelBuilder, LinearExpr};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Course<'a> {
    pub code: CourseCode,
    pub credits: i64,
    pub required: bool,
    pub geneds: Vec<&'a str>,
    pub elective_group: Option<&'a str>,
    pub prereqs: CourseReq,
}

/// Build the model and return (model, vars, total_credits LinearExpr)
pub fn build_model<'a>(
    courses: &'a [Course<'a>],
    num_semesters: usize,
    max_credits_per_semester: i64,
    min_credits: Option<i64>,
    geneds: Option<&'a [crate::geneds::GenEd]>,
    catalog: Option<&'a Catalog>,
) -> (
    CpModelBuilder,
    Vec<Vec<cp_sat::builder::BoolVar>>,
    LinearExpr,
) {
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
    let idx_map: HashMap<_, _> = courses
        .iter()
        .enumerate()
        .map(|(i, c)| (c.code.clone(), i))
        .collect();
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
        use crate::prereqs::CourseReq::*;
        match req {
            NotRequired => {}
            And(reqs) => {
                for r in reqs {
                    add_prereq_constraints(
                        model, vars, idx_map, courses, course_idx, r, num_semesters,
                    );
                }
            }
            Or(reqs) => {
                for s in 0..num_semesters {
                    let cur = vars[course_idx][s];
                    let mut or_exprs = Vec::new();
                    for r in reqs {
                        let or_var = model.new_bool_var();
                        match r {
                            PreCourse(code) => {
                                if let Some(&pre_idx) = idx_map.get(code) {
                                    if s == 0 {
                                    } else {
                                        let earlier_vars: Vec<_> = vars[pre_idx][..s].iter().copied().collect();
                                        if !earlier_vars.is_empty() {
                                            let sum_earlier: LinearExpr = earlier_vars.into_iter().collect();
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
                                add_prereq_constraints(
                                    model, vars, idx_map, courses, course_idx, r, num_semesters,
                                );
                            }
                            _ => unimplemented!("Only PreCourse, CoCourse, And, Or supported"),
                        }
                        or_exprs.push(or_var);
                    }
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
        add_prereq_constraints(
            &mut model, &vars, &idx_map, courses, i, &c.prereqs, num_semesters,
        );
    }
    // Gen-ed requirements (placeholder, real logic in main)
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
    // Elective group requirements (placeholder)
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
        let weighted_terms: Vec<(i64, _)> = courses
            .iter()
            .enumerate()
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
