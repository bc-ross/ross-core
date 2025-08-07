use crate::prereqs::CourseReq::{self, *};
use crate::schedule::CourseCode;
use std::collections::HashMap;
use varisat::ExtendFormula;
use varisat::{CnfFormula, Lit, Var, solver::Solver};

/// Recursively encode a CourseReq into CNF, returning a variable that is true iff the req is satisfied.
/// Uses Tseitin transformation for nested logic.
fn encode_course_req(
    req: &CourseReq,
    var_map: &mut HashMap<CourseCode, Var>,
    formula: &mut CnfFormula,
    aux_vars: &mut Vec<Var>,
) -> Var {
    match req {
        CourseReq::And(reqs) => {
            let this_var = new_aux_var(aux_vars);
            let sub_vars: Vec<Var> = reqs
                .iter()
                .map(|sub| encode_course_req(sub, var_map, formula, aux_vars))
                .collect();
            // this_var <=> (sub1 & sub2 & ...)
            // (¬this_var ∨ sub1), (¬this_var ∨ sub2), ..., (¬sub1 ∨ ¬sub2 ∨ ... ∨ this_var)
            for &v in &sub_vars {
                formula.add_clause(&[!Lit::from_var(this_var, false), Lit::from_var(v, false)]);
            }
            let mut clause = sub_vars
                .iter()
                .map(|&v| !Lit::from_var(v, false))
                .collect::<Vec<_>>();
            clause.push(Lit::from_var(this_var, false));
            formula.add_clause(&clause);
            this_var
        }
        CourseReq::Or(reqs) => {
            let this_var = new_aux_var(aux_vars);
            let sub_vars: Vec<Var> = reqs
                .iter()
                .map(|sub| encode_course_req(sub, var_map, formula, aux_vars))
                .collect();
            // this_var <=> (sub1 | sub2 | ...)
            // (¬sub1 ∨ this_var), (¬sub2 ∨ this_var), ..., (¬this_var ∨ sub1 ∨ sub2 ∨ ...)
            for &v in &sub_vars {
                formula.add_clause(&[Lit::from_var(this_var, false), !Lit::from_var(v, false)]);
            }
            let mut clause = sub_vars
                .iter()
                .map(|&v| Lit::from_var(v, false))
                .collect::<Vec<_>>();
            clause.push(!Lit::from_var(this_var, false));
            formula.add_clause(&clause);
            this_var
        }
        CourseReq::PreCourse(code)
        | CourseReq::CoCourse(code)
        | CourseReq::PreCourseGrade(code, _)
        | CourseReq::CoCourseGrade(code, _) => {
            // Each course gets a variable, true if taken
            let var = *var_map
                .entry(code.clone())
                .or_insert_with(|| new_aux_var(aux_vars));
            var
        }
        _ => {
            // For Program, Instructor, None: always satisfied (true)
            let var = new_aux_var(aux_vars);
            // Add unit clause: var is true
            formula.add_clause(&[Lit::from_var(var, false)]);
            var
        }
    }
}

fn new_aux_var(aux_vars: &mut Vec<Var>) -> Var {
    let v = Var::from_index(aux_vars.len());
    aux_vars.push(v);
    v
}

pub fn test_prereq_sat() {
    // Example: (A OR B) AND C
    let a = CourseCode {
        stem: "MATH".to_string(),
        code: 1010.into(),
    };
    let b = CourseCode {
        stem: "PHYS".to_string(),
        code: 2010.into(),
    };
    let c = CourseCode {
        stem: "CHEM".to_string(),
        code: 1210.into(),
    };

    let req = CourseReq::And(vec![
        CourseReq::Or(vec![
            CourseReq::PreCourse(a.clone()),
            CourseReq::PreCourse(b.clone()),
        ]),
        CourseReq::PreCourse(c.clone()),
    ]);

    let mut var_map = HashMap::new();
    let mut aux_vars = Vec::new();
    let mut formula = CnfFormula::new();

    let root_var = encode_course_req(&req, &mut var_map, &mut formula, &mut aux_vars);
    // Require the root variable to be true (i.e., the whole prereq is satisfied)
    formula.add_clause(&[Lit::from_var(root_var, false)]);

    let mut solver = Solver::new();
    solver.add_formula(&formula);

    if solver.solve().unwrap() {
        let model = solver.model().unwrap();
        println!("SAT solution found:");
        println!("{:?}", var_map);
        println!("{:?}", model);
        for (code, var) in &var_map {
            let idx = var.index();
            if idx < model.len() && model[idx].is_positive() {
                println!("Take course: {}", code);
            }
        }
    } else {
        println!("No solution found.");
    }
}
