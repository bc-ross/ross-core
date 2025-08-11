use or_tools::sat::*;
use std::collections::HashMap;
use or_tools::sat::cp_model::*;

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

    // Create model
    let mut model = CpModel::new();

    // Create boolean vars for course in semester
    let mut c_vars: Vec<Vec<IntVar>> = Vec::with_capacity(courses.len());
    for _ in 0..courses.len() {
        let mut sem_vars = Vec::with_capacity(num_semesters);
        for s in 0..num_semesters {
            sem_vars.push(model.new_bool_var(&format!("c_{}_sem_{}", c_vars.len(), s)));
        }
        c_vars.push(sem_vars);
    }

    // Each required course must be taken exactly once
    for (i, course) in courses.iter().enumerate() {
        if course.required {
            let vars = &c_vars[i];
            model.add_exactly_one(vars);
        }
    }

    // Optional courses: at most one semester
    for (i, course) in courses.iter().enumerate() {
        if !course.required {
            let vars = &c_vars[i];
            model.add_at_most_one(vars);
        }
    }

    // Prerequisite ordering constraints
    for (i, course) in courses.iter().enumerate() {
        for &pre in &course.prereqs {
            let pre_i = *course_idx.get(pre).unwrap();
            for s in 0..num_semesters {
                if s == 0 {
                    model.add_equality(&c_vars[i][0], 0);
                } else {
                    // c_vars[i][s] => OR of c_vars[pre_i][0..s-1]
                    let pre_before_s = &c_vars[pre_i][..s];
                    let sum_pre: IntExpr =
                        pre_before_s.iter().cloned().map(IntVar::into_expr).sum();
                    model.add_implication_with_at_least(&c_vars[i][s], sum_pre.ge(1));
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
                    gened_vars.push(c_vars[i][s].clone());
                }
            }
        }
        model.add_at_least_k(gened_vars, count);
    }

    // Elective group requirements
    for (&grp, &count) in &elective_reqs {
        let mut elective_vars = Vec::new();
        for (i, course) in courses.iter().enumerate() {
            if course.elective_group == Some(grp) {
                for s in 0..num_semesters {
                    elective_vars.push(c_vars[i][s].clone());
                }
            }
        }
        model.add_at_least_k(elective_vars, count);
    }

    // Per-semester credit limit
    for s in 0..num_semesters {
        let mut weighted_vars = Vec::new();
        for (i, course) in courses.iter().enumerate() {
            weighted_vars.push((c_vars[i][s].clone(), course.credits));
        }
        model.add_weighted_sum_less_or_equal(weighted_vars, max_credits_per_sem);
    }

    // Objective: minimize total credits
    let mut obj_terms = Vec::new();
    for (i, course) in courses.iter().enumerate() {
        for s in 0..num_semesters {
            obj_terms.push((c_vars[i][s].clone(), course.credits));
        }
    }
    model.minimize_weighted_sum(obj_terms);

    // Solve
    let solver = CpSolver::new();
    let status = solver.solve(&model);

    if status == CpSolverStatus::Optimal || status == CpSolverStatus::Feasible {
        println!("Schedule found:");
        let solution = solver.solution().expect("Expected solution");

        for s in 0..num_semesters {
            println!("Semester {}", s + 1);
            let mut sem_credits = 0;
            for (i, course) in courses.iter().enumerate() {
                if solution.value(&c_vars[i][s]) > 0 {
                    println!("  {} ({} credits)", course.id, course.credits);
                    sem_credits += course.credits;
                }
            }
            println!("  Credits: {}", sem_credits);
        }
        println!("Total credits: {}", solution.objective_value());
    } else {
        println!("No solution found.");
    }
}
