use or_tools::sat::cp_model::*;
use or_tools::sat::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Course {
    id: &'static str,
    credits: i64,
    required: bool,
    geneds: Vec<&'static str>,
    elective_group: Option<&'static str>,
    prereqs: Vec<&'static str>,
}

fn main() {
    let courses = vec![
        Course {
            id: "MATH101",
            credits: 4,
            required: true,
            geneds: vec![],
            elective_group: None,
            prereqs: vec![],
        },
        Course {
            id: "MATH102",
            credits: 4,
            required: true,
            geneds: vec![],
            elective_group: None,
            prereqs: vec!["MATH101"],
        },
        Course {
            id: "CS101",
            credits: 3,
            required: true,
            geneds: vec![],
            elective_group: None,
            prereqs: vec![],
        },
        Course {
            id: "CS201",
            credits: 3,
            required: true,
            geneds: vec![],
            elective_group: None,
            prereqs: vec!["CS101"],
        },
        Course {
            id: "ENG001",
            credits: 3,
            required: false,
            geneds: vec!["WRI"],
            elective_group: None,
            prereqs: vec![],
        },
        Course {
            id: "PHIL01",
            credits: 3,
            required: false,
            geneds: vec!["HUM"],
            elective_group: None,
            prereqs: vec![],
        },
        Course {
            id: "BIO01",
            credits: 4,
            required: false,
            geneds: vec!["SCI"],
            elective_group: None,
            prereqs: vec![],
        },
        Course {
            id: "ART01",
            credits: 3,
            required: false,
            geneds: vec!["ART"],
            elective_group: None,
            prereqs: vec![],
        },
        Course {
            id: "ELEC_A1",
            credits: 2,
            required: false,
            geneds: vec![],
            elective_group: Some("ELEC_A"),
            prereqs: vec![],
        },
        Course {
            id: "ELEC_A2",
            credits: 3,
            required: false,
            geneds: vec![],
            elective_group: Some("ELEC_A"),
            prereqs: vec![],
        },
        Course {
            id: "CS301",
            credits: 3,
            required: false,
            geneds: vec![],
            elective_group: None,
            prereqs: vec!["CS201"],
        },
    ];

    let gened_reqs: HashMap<&str, i64> = [("WRI", 1), ("HUM", 1), ("SCI", 1)]
        .iter()
        .cloned()
        .collect();
    let elective_reqs: HashMap<&str, i64> = [("ELEC_A", 1)].iter().cloned().collect();

    let num_semesters = 8;
    let max_credits_per_sem = 18;

    let mut course_idx: HashMap<&str, usize> = HashMap::new();
    for (i, course) in courses.iter().enumerate() {
        course_idx.insert(course.id, i);
    }

    // Create model builder
    let mut model = cp_model_builder();

    // Create boolean vars for course in semester
    let mut c_vars: Vec<Vec<IntVar>> = Vec::with_capacity(courses.len());
    for i in 0..courses.len() {
        let mut sem_vars = Vec::with_capacity(num_semesters);
        for s in 0..num_semesters {
            sem_vars.push(model.new_bool_var().with_name(&format!("c_{}_sem_{}", i, s)));
        }
        c_vars.push(sem_vars);
    }

    // Each required course must be taken exactly once
    for (i, course) in courses.iter().enumerate() {
        if course.required {
            model.add_exactly_one(&c_vars[i]);
        }
    }

    // Optional courses: at most one semester
    for (i, course) in courses.iter().enumerate() {
        if !course.required {
            model.add_at_most_one(&c_vars[i]);
        }
    }

    // Prerequisite ordering constraints
    for (i, course) in courses.iter().enumerate() {
        for &pre in &course.prereqs {
            let pre_i = *course_idx.get(pre).unwrap();
            for s in 0..num_semesters {
                if s == 0 {
                    // If course is taken in first semester, prerequisite cannot be satisfied
                    model.add_equality(c_vars[i][0], 0);
                } else {
                    // c_vars[i][s] => OR of c_vars[pre_i][0..s-1]
                    let mut pre_vars = Vec::new();
                    for t in 0..s {
                        pre_vars.push(c_vars[pre_i][t]);
                    }
                    if !pre_vars.is_empty() {
                        let pre_sum = linear_expr_sum(&pre_vars);
                        model.add_implication(c_vars[i][s], pre_sum.ge(1));
                    }
                }
            }
        }
    }

    // Gen-ed requirements
    for (&gened, &count) in &gened_reqs {
        let mut gened_vars = Vec::new();
        for (i, course) in courses.iter().enumerate() {
            if course.geneds.contains(&gened) {
                for s in 0..num_semesters {
                    gened_vars.push(c_vars[i][s]);
                }
            }
        }
        if !gened_vars.is_empty() {
            let sum = linear_expr_sum(&gened_vars);
            model.add_linear_constraint(sum, Domain::from(count..));
        }
    }

    // Elective group requirements
    for (&grp, &count) in &elective_reqs {
        let mut elective_vars = Vec::new();
        for (i, course) in courses.iter().enumerate() {
            if course.elective_group == Some(grp) {
                for s in 0..num_semesters {
                    elective_vars.push(c_vars[i][s]);
                }
            }
        }
        if !elective_vars.is_empty() {
            let sum = linear_expr_sum(&elective_vars);
            model.add_linear_constraint(sum, Domain::from(count..));
        }
    }

    // Per-semester credit limit
    for s in 0..num_semesters {
        let mut vars = Vec::new();
        let mut coeffs = Vec::new();
        for (i, course) in courses.iter().enumerate() {
            vars.push(c_vars[i][s]);
            coeffs.push(course.credits);
        }
        model.add_linear_constraint_with_coeffs(&vars, &coeffs, Domain::from(..=max_credits_per_sem));
    }

    // Objective: minimize total credits
    let credit_terms: Vec<LinearExpr> = courses.iter().enumerate()
        .flat_map(|(i, course)| {
            (0..num_semesters).map(move |s| LinearExpr::from(c_vars[i][s]) * course.credits)
        })
        .collect();
    let total_credits = LinearExpr::sum(credit_terms);
    model.minimize(total_credits);

    // Solve
    let model = model.build();
    let response = solve_cp_model(&model);

    match response.status() {
        CpSolverStatus::Optimal | CpSolverStatus::Feasible => {
            println!("Schedule found:");

            for s in 0..num_semesters {
                println!("Semester {}", s + 1);
                let mut sem_credits = 0;
                for (i, course) in courses.iter().enumerate() {
                    if response.boolean_value(c_vars[i][s]) {
                        println!("  {} ({} credits)", course.id, course.credits);
                        sem_credits += course.credits;
                    }
                }
                println!("  Credits: {}", sem_credits);
            }
            println!("Total credits: {}", response.objective_value());
        }
        _ => {
            println!("No solution found. Status: {:?}", response.status());
        }
    }
}
