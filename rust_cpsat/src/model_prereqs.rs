//! Functions for adding prerequisite constraints.
use crate::model_context::{ModelBuilderContext, Course};
use std::collections::HashMap;
use crate::schedule::CourseCode;
use crate::prereqs::CourseReq;

pub fn add_prereq_constraints<'a>(ctx: &mut ModelBuilderContext<'a>) {
    let idx_map: HashMap<_, _> = ctx.courses.iter().enumerate().map(|(i, c)| (c.code.clone(), i)).collect();
    for (i, c) in ctx.courses.iter().enumerate() {
        add_prereq_for_course(ctx, &idx_map, i, &c.prereqs);
    }
}

fn add_prereq_for_course<'a>(ctx: &mut ModelBuilderContext<'a>, idx_map: &HashMap<CourseCode, usize>, course_idx: usize, req: &CourseReq) {
    use crate::prereqs::CourseReq::*;
    let num_semesters = ctx.num_semesters;
    match req {
        NotRequired => {}
        And(reqs) => {
            for r in reqs {
                add_prereq_for_course(ctx, idx_map, course_idx, r);
            }
        }
        Or(reqs) => {
            for s in 0..num_semesters {
                let cur = ctx.vars[course_idx][s];
                let mut or_exprs = Vec::new();
                for r in reqs {
                    let or_var = ctx.model.new_bool_var();
                    match r {
                        PreCourse(code) => {
                            if let Some(&pre_idx) = idx_map.get(code) {
                                if s == 0 {
                                } else {
                                    let earlier_vars: Vec<_> = ctx.vars[pre_idx][..s].iter().copied().collect();
                                    if !earlier_vars.is_empty() {
                                        let sum_earlier = earlier_vars.into_iter().collect();
                                        ctx.model.add_linear_constraint(sum_earlier - or_var, [(0, i64::MAX)]);
                                        or_exprs.push(or_var);
                                    }
                                }
                            }
                        }
                        CoCourse(code) => {
                            if let Some(&co_idx) = idx_map.get(code) {
                                let upto_vars: Vec<_> = ctx.vars[co_idx][..=s].iter().copied().collect();
                                if !upto_vars.is_empty() {
                                    let sum_upto = upto_vars.into_iter().collect();
                                    ctx.model.add_linear_constraint(sum_upto - or_var, [(0, i64::MAX)]);
                                    or_exprs.push(or_var);
                                }
                            }
                        }
                        And(_) | Or(_) => {
                            add_prereq_for_course(ctx, idx_map, course_idx, r);
                        }
                        _ => unimplemented!("Only PreCourse, CoCourse, And, Or supported"),
                    }
                    or_exprs.push(or_var);
                }
                if !or_exprs.is_empty() {
                    let sum_or = or_exprs.iter().copied().collect();
                    ctx.model.add_linear_constraint(sum_or - cur, [(0, i64::MAX)]);
                }
            }
        }
        PreCourse(code) => {
            if let Some(&pre_idx) = idx_map.get(code) {
                for s in 0..num_semesters {
                    let cur = ctx.vars[course_idx][s];
                    if s == 0 {
                        ctx.model.add_eq(cur, 0);
                    } else {
                        let earlier_vars: Vec<_> = ctx.vars[pre_idx][..s].iter().copied().collect();
                        if !earlier_vars.is_empty() {
                            let sum_earlier = earlier_vars.into_iter().collect();
                            ctx.model.add_linear_constraint(sum_earlier - cur, [(0, i64::MAX)]);
                        } else {
                            ctx.model.add_eq(cur, 0);
                        }
                    }
                }
            } else {
                for s in 0..num_semesters {
                    ctx.model.add_eq(ctx.vars[course_idx][s], 0);
                }
            }
        }
        CoCourse(code) => {
            if let Some(&co_idx) = idx_map.get(code) {
                for s in 0..num_semesters {
                    let cur = ctx.vars[course_idx][s];
                    let upto_vars: Vec<_> = ctx.vars[co_idx][..=s].iter().copied().collect();
                    if !upto_vars.is_empty() {
                        let sum_upto = upto_vars.into_iter().collect();
                        ctx.model.add_linear_constraint(sum_upto - cur, [(0, i64::MAX)]);
                    } else {
                        ctx.model.add_eq(cur, 0);
                    }
                }
            } else {
                for s in 0..num_semesters {
                    ctx.model.add_eq(ctx.vars[course_idx][s], 0);
                }
            }
        }
        _ => unimplemented!("Only PreCourse, CoCourse, And, Or supported"),
    }
}
