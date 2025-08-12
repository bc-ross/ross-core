//! Functions for adding GenEd constraints.
use crate::model_context::Course;
use crate::model_context::ModelBuilderContext;
use crate::geneds::{GenEd, GenEdReq};
use cp_sat::builder::LinearExpr;

/// Add GenEd constraints to the model.
pub fn add_gened_constraints<'a>(ctx: &mut ModelBuilderContext<'a>) {
    let model = &mut ctx.model;
    let courses = &ctx.courses;
    let vars = &ctx.vars;
    let num_semesters = ctx.num_semesters;
    let geneds = match ctx.catalog {
        Some(catalog) => &catalog.geneds,
        None => return,
    };

    // Helper: for a course code, find its index in flat_courses
    let code_to_idx: std::collections::HashMap<_, _> = courses
        .iter()
        .enumerate()
        .map(|(i, course)| (course.code.clone(), i))
        .collect();

    // Helper: for a course code, return a variable that is 1 if the course is scheduled in any semester
    let course_in_schedule = |idx: usize| {
        let mut expr = LinearExpr::from(0);
        for s in 0..num_semesters {
            expr = expr + LinearExpr::from(vars[idx][s].clone());
        }
        expr
    };

    // Helper: for a set of course codes, return a vector of their indices (if all present)
    let codes_to_indices = |codes: &Vec<crate::schedule::CourseCode>| -> Option<Vec<usize>> {
        codes.iter().map(|c| code_to_idx.get(c).copied()).collect()
    };

    // --- Core GenEds: no overlap restrictions ---
    for gened in geneds.iter() {
        if let GenEd::Core { req, .. } = gened {
            match req {
                GenEdReq::Set(codes) => {
                    if let Some(indices) = codes_to_indices(codes) {
                        for idx in indices {
                            // Each course must be scheduled somewhere
                            model.add_ge(course_in_schedule(idx), LinearExpr::from(1));
                        }
                    }
                }
                GenEdReq::SetOpts(opts) => {
                    // At least one option set must be fully present
                    let mut option_exprs = Vec::new();
                    for opt in opts {
                        if let Some(indices) = codes_to_indices(opt) {
                            // All courses in this option must be present
                            let mut all_present = LinearExpr::from(0);
                            for idx in indices {
                                all_present = all_present + course_in_schedule(idx);
                            }
                            // If all are present, sum == len
                            // Use a bool var to represent this option
                            let opt_var = model.new_bool_var();
                            // model.add_le(all_present.clone(), LinearExpr::from(opt.len() as i64) + (LinearExpr::from(1) - opt_var.clone()) * (opt.len() as i64));
                            // model.add_ge(all_present, LinearExpr::from(opt.len() as i64) - (LinearExpr::from(1) - opt_var.clone()) * (opt.len() as i64));
                            // Instead, expand (LinearExpr * n) as repeated addition
                            let mut le_expr = LinearExpr::from(1) - opt_var.clone();
                            let mut le_scaled = LinearExpr::from(0);
                            for _ in 0..opt.len() {
                                le_scaled = le_scaled + le_expr.clone();
                            }
                            model.add_le(all_present.clone(), LinearExpr::from(opt.len() as i64) + le_scaled);
                            let mut ge_scaled = LinearExpr::from(0);
                            for _ in 0..opt.len() {
                                ge_scaled = ge_scaled + le_expr.clone();
                            }
                            model.add_ge(all_present, LinearExpr::from(opt.len() as i64) - ge_scaled);
                            option_exprs.push(opt_var);
                        }
                    }
                    if !option_exprs.is_empty() {
                        let mut sum = LinearExpr::from(0);
                        for v in option_exprs { sum = sum + LinearExpr::from(v); }
                        model.add_ge(sum, LinearExpr::from(1));
                    }
                }
                GenEdReq::Courses { num, courses } => {
                    if let Some(indices) = codes_to_indices(courses) {
                        let mut sum = LinearExpr::from(0);
                        for idx in indices { sum = sum + course_in_schedule(idx); }
                        model.add_ge(sum, LinearExpr::from(*num as i64));
                    }
                }
                GenEdReq::Credits { num, courses } => {
                    if let Some(indices) = codes_to_indices(courses) {
                        let mut sum = LinearExpr::from(0);
                        for idx in indices.iter() {
                            let credits = ctx.courses[*idx].credits;
                            // sum = sum + course_in_schedule(*idx) * credits;
                            if credits > 0 {
                                for _ in 0..credits {
                                    sum = sum + course_in_schedule(*idx);
                                }
                            } else if credits < 0 {
                                for _ in 0..(-credits) {
                                    sum = sum - course_in_schedule(*idx);
                                }
                            }
                        }
                        model.add_ge(sum, LinearExpr::from(*num as i64));
                    }
                }
            }
        }
    }

    // --- Foundation GenEds: no course may satisfy more than one Foundation ---
    // For each Foundation, build a set of eligible courses
    let mut foundation_sets = Vec::new();
    for gened in geneds.iter() {
        if let GenEd::Foundation { req, .. } = gened {
            match req {
                GenEdReq::Set(codes) | GenEdReq::Courses { courses: codes, .. } | GenEdReq::Credits { courses: codes, .. } => {
                    if let Some(indices) = codes_to_indices(codes) {
                        foundation_sets.push(indices);
                    }
                }
                GenEdReq::SetOpts(opts) => {
                    // Flatten all options into one set for overlap constraint
                    let mut indices = Vec::new();
                    for opt in opts {
                        if let Some(opt_indices) = codes_to_indices(opt) {
                            indices.extend(opt_indices);
                        }
                    }
                    foundation_sets.push(indices);
                }
            }
        }
    }
    // For each course, count how many Foundations it could satisfy
    let mut course_foundation_counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for (fidx, set) in foundation_sets.iter().enumerate() {
        for &idx in set {
            *course_foundation_counts.entry(idx).or_insert(0) += 1;
        }
    }
    // For each course, add constraint: sum of Foundation assignments <= 1
    for (&idx, &count) in &course_foundation_counts {
        if count > 1 {
            // For each Foundation, create a bool var if this course is used for that Foundation
            let mut used_vars = Vec::new();
            for set in &foundation_sets {
                if set.contains(&idx) {
                    let used = model.new_bool_var();
                    // If course is scheduled, used can be 1, else 0
                    model.add_le(used.clone(), course_in_schedule(idx));
                    used_vars.push(used);
                }
            }
            // At most one Foundation can use this course
            let mut sum = LinearExpr::from(0);
            for v in used_vars { sum = sum + LinearExpr::from(v); }
            model.add_le(sum, LinearExpr::from(1));
        }
    }

    // --- Skills & Perspectives: no course may satisfy more than 3 S&Ps ---
    let mut sp_sets = Vec::new();
    for gened in geneds.iter() {
        if let GenEd::SkillAndPerspective { req, .. } = gened {
            match req {
                GenEdReq::Set(codes) | GenEdReq::Courses { courses: codes, .. } | GenEdReq::Credits { courses: codes, .. } => {
                    if let Some(indices) = codes_to_indices(codes) {
                        sp_sets.push(indices);
                    }
                }
                GenEdReq::SetOpts(opts) => {
                    let mut indices = Vec::new();
                    for opt in opts {
                        if let Some(opt_indices) = codes_to_indices(opt) {
                            indices.extend(opt_indices);
                        }
                    }
                    sp_sets.push(indices);
                }
            }
        }
    }
    // For each course, count how many S&Ps it could satisfy
    let mut course_sp_counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for set in sp_sets.iter() {
        for &idx in set {
            *course_sp_counts.entry(idx).or_insert(0) += 1;
        }
    }
    // For each course, add constraint: sum of S&P assignments <= 3
    for (&idx, &count) in &course_sp_counts {
        if count > 3 {
            // For each S&P, create a bool var if this course is used for that S&P
            let mut used_vars = Vec::new();
            for set in &sp_sets {
                if set.contains(&idx) {
                    let used = model.new_bool_var();
                    model.add_le(used.clone(), course_in_schedule(idx));
                    used_vars.push(used);
                }
            }
            let mut sum = LinearExpr::from(0);
            for v in used_vars { sum = sum + LinearExpr::from(v); }
            model.add_le(sum, LinearExpr::from(3));
        }
    }
}
