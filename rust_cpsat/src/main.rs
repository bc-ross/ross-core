use cp_sat::builder::CpModelBuilder;
use cp_sat::proto::CpSolverStatus;

struct Course<'a> {
    id: &'a str,
    credits: i64,
    required: bool,
    geneds: Vec<&'a str>,
    elective_group: Option<&'a str>,
    prereqs: Vec<&'a str>,
}

fn main() {
    let courses = vec![
        Course { id: "MATH101", credits: 4, required: true, geneds: vec![], elective_group: None, prereqs: vec![] },
        Course { id: "MATH102", credits: 4, required: true, geneds: vec![], elective_group: None, prereqs: vec!["MATH101"] },
        Course { id: "CS101", credits: 3, required: true, geneds: vec![], elective_group: None, prereqs: vec![] },
        Course { id: "CS201", credits: 3, required: true, geneds: vec![], elective_group: None, prereqs: vec!["CS101"] },
        Course { id: "ENG001", credits: 3, required: false, geneds: vec!["WRI"], elective_group: None, prereqs: vec![] },
        Course { id: "PHIL01", credits: 3, required: false, geneds: vec!["HUM"], elective_group: None, prereqs: vec![] },
        Course { id: "BIO01", credits: 4, required: false, geneds: vec!["SCI"], elective_group: None, prereqs: vec![] },
        Course { id: "ART01", credits: 3, required: false, geneds: vec!["ART"], elective_group: None, prereqs: vec![] },
        Course { id: "ELEC_A1", credits: 2, required: false, geneds: vec![], elective_group: Some("ELEC_A"), prereqs: vec![] },
        Course { id: "ELEC_A2", credits: 3, required: false, geneds: vec![], elective_group: Some("ELEC_A"), prereqs: vec![] },
        Course { id: "CS301", credits: 3, required: false, geneds: vec![], elective_group: None, prereqs: vec!["CS201"] },
    ];

    let gened_reqs = vec![("WRI", 1), ("HUM", 1), ("SCI", 1)];
    let elective_reqs = vec![("ELEC_A", 1)];

    let num_semesters = 8;
    let mut model = CpModelBuilder::default();
    let mut vars = Vec::new();

    for (i, _) in courses.iter().enumerate() {
        let mut sem_vars = Vec::new();
        for s in 0..num_semesters {
            let v = model.new_bool_var(format!("c_{}_{}", i, s));
            sem_vars.push(v);
        }
        vars.push(sem_vars);
    }

    // Required courses exactly once
    for (i, c) in courses.iter().enumerate() {
        if c.required {
            model.add_exactly_one(&vars[i]);
        }
    }

    // Optional courses at most once
    for (i, c) in courses.iter().enumerate() {
        if !c.required {
            model.add_at_most_one(&vars[i]);
        }
    }

    // Prereqs: each course variable in semester s implies some prereq var in an earlier semester
    let idx_map: std::collections::HashMap<_, _> =
        courses.iter().enumerate().map(|(i, c)| (c.id, i)).collect();
    for (i, c) in courses.iter().enumerate() {
        for &pre in &c.prereqs {
            let pi = idx_map[pre];
            for s in 0..num_semesters {
                let cur = vars[i][s];
                if s == 0 {
                    model.add_equality(cur, 0);
                } else {
                    let earlier: Vec<_> = vars[pi][..s].iter().cloned().collect();
                    model.add_implication(cur, model.linear_expr_sum(earlier) >= 1);
                }
            }
        }
    }

    // Gen-ed and elective requirements
    for &(g, req) in &gened_reqs {
        let mut all = Vec::new();
        for (i, c) in courses.iter().enumerate() {
            if c.geneds.contains(&g) {
                all.extend(vars[i].clone());
            }
        }
        model.add_at_least_k(&all, req);
    }
    for &(eg, req) in &elective_reqs {
        let mut all = Vec::new();
        for (i, c) in courses.iter().enumerate() {
            if c.elective_group == Some(eg) {
                all.extend(vars[i].clone());
            }
        }
        model.add_at_least_k(&all, req);
    }

    // Semester credit caps
    for s in 0..num_semesters {
        let terms: Vec<(cp_sat::builder::BoolVar, i64)> = courses.iter().enumerate()
            .map(|(i, c)| (vars[i][s].clone(), c.credits))
            .collect();
        model.add_weighted_sum(&terms, 0, 18);
    }

    // Objective: minimize total credits
    let obj_terms: Vec<(cp_sat::builder::BoolVar, i64)> = courses.iter().enumerate()
        .flat_map(|(i, c)| (0..num_semesters).map(move |s| (vars[i][s].clone(), c.credits)))
        .collect();
    model.minimize(obj_terms);

    // Solve and report
    let resp = model.solve();
    if resp.status() == CpSolverStatus::Optimal || resp.status() == CpSolverStatus::Feasible {
        let total: i64 = obj_terms.iter()
            .filter(|(v, _)| v.value(&resp) == 1)
            .map(|(_, w)| w)
            .sum();
        println!("Total credits: {}", total);
        for s in 0..num_semesters {
            println!("Semester {}", s + 1);
            for (i, c) in courses.iter().enumerate() {
                if vars[i][s].value(&resp) == 1 {
                    println!("  {} ({} cr)", c.id, c.credits);
                }
            }
        }
    } else {
        println!("No feasible solution found.");
    }
}
