//! Context struct for model building and shared state.
use crate::prereqs::CourseReq;
use crate::schedule::{Catalog, CourseCode, Schedule};
use cp_sat::builder::{BoolVar, CpModelBuilder, IntVar, LinearExpr};

#[derive(Clone)]
pub struct Course<'a> {
    pub code: CourseCode,
    pub credits: i64,
    pub required: bool,
    pub geneds: Vec<&'a str>,
    pub elective_group: Option<&'a str>,
    pub prereqs: CourseReq,
}

pub struct ModelBuilderContext<'a> {
    pub model: CpModelBuilder,
    pub vars: Vec<Vec<BoolVar>>,
    pub courses: Vec<Course<'a>>,
    pub num_semesters: usize,
    pub max_credits_per_semester: i64,
    pub min_credits: Option<i64>,
    pub geneds: Option<&'a [crate::geneds::GenEd]>,
    pub catalog: Option<&'a Catalog>,
    pub incoming_codes: Vec<CourseCode>,
    pub program_electives: Vec<&'a crate::schedule::Elective>,
    pub semester_credit_vars: Vec<IntVar>,
}

impl<'a> ModelBuilderContext<'a> {
    /// Create a new ModelBuilderContext from a schedule and max credits per semester.
    pub fn new(sched: &'a Schedule, max_credits_per_semester: i64) -> Self {
        // Add incoming courses as semester 0
        let mut all_codes = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        // Add incoming courses first
        for code in &sched.incoming {
            all_codes.insert(code.clone());
        }
        // Add planned courses and their prereqs
        for sem in &sched.courses {
            for code in sem {
                all_codes.insert(code.clone());
                queue.push_back(code.clone());
            }
        }

        // Add GenEd-eligible courses
        for gened in &sched.catalog.geneds {
            use crate::geneds::{ElectiveReq, GenEd};
            let reqs: Vec<&ElectiveReq> = match gened {
                GenEd::Core { req, .. } => vec![req],
                GenEd::Foundation { req, .. } => vec![req],
                GenEd::SkillAndPerspective { req, .. } => vec![req],
            };
            for req in reqs {
                match req {
                    ElectiveReq::Set(codes)
                    | ElectiveReq::Courses { courses: codes, .. }
                    | ElectiveReq::Credits { courses: codes, .. } => {
                        for code in codes {
                            all_codes.insert(code.clone());
                            queue.push_back(code.clone());
                        }
                    }
                    ElectiveReq::SetOpts(opts) => {
                        for opt in opts {
                            for code in opt {
                                all_codes.insert(code.clone());
                                queue.push_back(code.clone());
                            }
                        }
                    }
                }
            }
        }
        // Collect program electives for the selected programs
        let mut program_electives: Vec<&crate::schedule::Elective> = Vec::new();
        for prog_name in &sched.programs {
            if let Some(prog) = sched.catalog.programs.iter().find(|p| &p.name == prog_name) {
                for elective in &prog.electives {
                    program_electives.push(elective);
                    // Also ensure elective courses are included in all_codes so they can be modelled
                    use crate::geneds::ElectiveReq;
                    match &elective.req {
                        ElectiveReq::Set(codes)
                        | ElectiveReq::Courses { courses: codes, .. }
                        | ElectiveReq::Credits { courses: codes, .. } => {
                            for code in codes {
                                all_codes.insert(code.clone());
                                queue.push_back(code.clone());
                            }
                        }
                        ElectiveReq::SetOpts(opts) => {
                            for opt in opts {
                                for code in opt {
                                    all_codes.insert(code.clone());
                                    queue.push_back(code.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        while let Some(code) = queue.pop_front() {
            if let Some(req) = sched.catalog.prereqs.get(&code) {
                fn collect_prereq_codes(
                    req: &CourseReq,
                    all_codes: &mut std::collections::HashSet<CourseCode>,
                    catalog: &Catalog,
                    queue: &mut std::collections::VecDeque<CourseCode>,
                ) {
                    use crate::prereqs::CourseReq::*;
                    match req {
                        And(reqs) | Or(reqs) => {
                            for r in reqs {
                                collect_prereq_codes(r, all_codes, catalog, queue);
                            }
                        }
                        PreCourse(code) | CoCourse(code) => {
                            if all_codes.insert(code.clone()) {
                                queue.push_back(code.clone());
                            }
                        }
                        _ => {}
                    }
                }
                collect_prereq_codes(req, &mut all_codes, &sched.catalog, &mut queue);
            }
        }

        // Build Course structs for all codes, and print diagnostics
        let mut courses = Vec::new();
        let mut total_credits = 0;
        println!("[DIAG] Courses included in model:");
        println!("[DIAG] incoming_codes: {:?}", sched.incoming);
        for code in &all_codes {
            let (credits, prereqs) = match sched.catalog.courses.get(code) {
                Some((_name, credits_opt, _offering)) => {
                    let credits = credits_opt.unwrap_or(0) as i64;
                    let prereqs = sched
                        .catalog
                        .prereqs
                        .get(code)
                        .cloned()
                        .unwrap_or(CourseReq::NotRequired);
                    (credits, prereqs)
                }
                None => (0, CourseReq::NotRequired),
            };
            total_credits += credits;
            let required = if sched.incoming.contains(code) {
                true
            } else if sched.courses.iter().flatten().any(|c| c == code) {
                true
            } else {
                false
            };
            println!(
                "  {} ({} credits) required: {} incoming: {}",
                code,
                credits,
                required,
                sched.incoming.contains(code)
            );
            courses.push(Course {
                code: code.clone(),
                credits,
                required,
                geneds: vec![],
                elective_group: None,
                prereqs,
            });
        }
        println!("[DIAG] Total courses: {}", courses.len());
        println!(
            "[DIAG] Total credits (all modeled courses): {}",
            total_credits
        );
        println!(
            "[DIAG] Semesters: {} (+1 for incoming)",
            sched.courses.len()
        );
        println!("[DIAG] Max credits/semester: {}", max_credits_per_semester);
        if !sched.catalog.geneds.is_empty() {
            println!("[DIAG] GenEd requirements:");
            for gened in &sched.catalog.geneds {
                use crate::geneds::GenEd;
                match gened {
                    GenEd::Core { name, req } => println!("  Core: {}: {:?}", name, req),
                    GenEd::Foundation { name, req } => {
                        println!("  Foundation: {}: {:?}", name, req)
                    }
                    GenEd::SkillAndPerspective { name, req } => {
                        println!("  S&P: {}: {:?}", name, req)
                    }
                }
            }
        }
        ModelBuilderContext {
            model: CpModelBuilder::default(),
            vars: Vec::new(),
            courses,
            num_semesters: sched.courses.len(), // already includes semester 0 after transformation
            max_credits_per_semester,
            min_credits: None,
            geneds: Some(&sched.catalog.geneds),
            catalog: Some(&sched.catalog),
            incoming_codes: sched.incoming.clone(),
            program_electives: program_electives,
            semester_credit_vars: Vec::new(),
        }
    }

    /// Set the minimum total credits constraint
    pub fn set_min_credits(&mut self, min_credits: i64) {
        self.min_credits = Some(min_credits);
    }

    /// Compute the total credits LinearExpr for the current context
    pub fn total_credits_expr(
        &self,
        vars: &Vec<Vec<BoolVar>>,
        flat_courses: &Vec<(Course, i64)>,
    ) -> LinearExpr {
        let mut obj_terms = Vec::new();
        for (i, (_course, credits)) in flat_courses.iter().enumerate() {
            // Skip semester 0 (incoming) when computing total scheduled credits
            for s in 1..self.num_semesters {
                obj_terms.push((*credits, vars[i][s]));
            }
        }
        obj_terms.into_iter().collect()
    }
}

/// Build the model pipeline: add variables, constraints, and return (model, vars, flat_courses)
pub fn build_model_pipeline<'a>(
    ctx: &mut ModelBuilderContext<'a>,
) -> (CpModelBuilder, Vec<Vec<BoolVar>>, Vec<(Course<'a>, i64)>) {
    crate::model_courses::add_courses(ctx);
    crate::model_semester::add_semester_constraints(ctx);
    crate::model_prereqs::add_prereq_constraints(ctx);
    crate::model_geneds::add_gened_constraints(ctx);
    // Build flat_courses as (Course, credits)
    let flat_courses = ctx.courses.iter().map(|c| (c.clone(), c.credits)).collect();
    (
        std::mem::replace(&mut ctx.model, CpModelBuilder::default()),
        ctx.vars.clone(),
        flat_courses,
    )
}
