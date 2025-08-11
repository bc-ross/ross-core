
mod model;
mod model_semester;
mod model_geneds;
mod model_courses;
mod model_context;
mod model_prereqs;

#[path = "../../src/schedule.rs"]
mod schedule;
#[path = "../../src/prereqs.rs"]
mod prereqs;
#[path = "../../src/geneds.rs"]
mod geneds;
#[path = "../../src/load_catalogs.rs"]
mod load_catalogs;

use crate::model::{ModelBuilderContext, build_model_pipeline};
use cp_sat::proto::CpSolverStatus;
use schedule::CourseCode;

fn main() {
    // --- Use a real catalog and fake schedule ---
    use crate::load_catalogs::CATALOGS;
    let catalog = &CATALOGS[0];
    let mut course_codes: Vec<CourseCode> = catalog.courses.keys().cloned().collect();
    let assigned: Vec<CourseCode> = course_codes.drain(0..8).collect();
    let mut semesters: Vec<Vec<CourseCode>> = vec![vec![]; 8];
    for (i, code) in assigned.into_iter().enumerate() {
        semesters[i].push(code);
    }
    let sched = schedule::Schedule {
        courses: semesters,
        programs: vec![],
        catalog: catalog.clone(),
    };
    println!("{:?}", &sched.courses);
    let max_credits_per_semester = 18;

    // Build the model using the modular pipeline
    let mut ctx = ModelBuilderContext::new(&sched, max_credits_per_semester);
    let (mut model, vars, flat_courses) = build_model_pipeline(&mut ctx);

    // Stage 1: minimize total credits
    let num_semesters = sched.courses.len();
    let total_credits = ctx.total_credits_expr(&vars, &flat_courses);
    model.minimize(total_credits.clone());
    let response = model.solve();

    // Compute min_credits as the sum of all scheduled (assigned + prereq) course credits in the solution
    let mut min_credits = None;
    if let CpSolverStatus::Optimal | CpSolverStatus::Feasible = response.status() {
        let mut total = 0;
        for (i, (_course, credits)) in flat_courses.iter().enumerate() {
            for s in 0..num_semesters {
                if vars[i][s].solution_value(&response) {
                    total += credits;
                }
            }
        }
        min_credits = Some(total);
    }

    // Add a constant to toggle the secondary objective
    const SPREAD_SEMESTERS: bool = true;

    // Stage 2: minimize spread, subject to min total credits
    if SPREAD_SEMESTERS {
        if let Some(min_credits) = min_credits {
            let mut ctx2 = ModelBuilderContext::new(&sched, max_credits_per_semester);
            ctx2.set_min_credits(min_credits);
            let (mut model2, vars2, flat_courses2) = build_model_pipeline(&mut ctx2);
            let total_credits_int: i64 = min_credits;
            let mean_load = total_credits_int / num_semesters as i64;
            let mut abs_deviation_vars = Vec::new();
            for s in 0..num_semesters {
                let semester_credits: i64 = flat_courses2
                    .iter()
                    .enumerate()
                    .map(|(i, (_course, credits))| {
                        if vars2[i][s].solution_value(&model2.solve()) { *credits } else { 0 }
                    })
                    .sum();
                let deviation = semester_credits - mean_load;
                abs_deviation_vars.push(deviation.abs());
            }
            let spread_penalty: i64 = abs_deviation_vars.iter().sum();
            println!("Lexicographic spread penalty: {}", spread_penalty);
            // Print schedule
            println!("Schedule found (lexicographic):");
            for s in 0..num_semesters {
                println!("Semester {}", s + 1);
                let mut sem_credits = 0;
                for (i, (course, credits)) in flat_courses2.iter().enumerate() {
                    if vars2[i][s].solution_value(&model2.solve()) {
                        println!("  {} ({} credits)", course.code, credits);
                        sem_credits += credits;
                    }
                }
                println!("  Credits: {}", sem_credits);
            }
            println!("Total credits: {}", min_credits);
            match crate::geneds::are_geneds_satisfied(&sched) {
                Ok(true) => println!("All GenEds satisfied!"),
                Ok(false) => println!("GenEd requirements NOT satisfied!"),
                Err(e) => println!("GenEd check error: {}", e),
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
                for (i, (course, credits)) in flat_courses.iter().enumerate() {
                    if vars[i][s].solution_value(&response) {
                        println!("  {} ({} credits)", course.code, credits);
                        sem_credits += credits;
                    }
                }
                println!("  Credits: {}", sem_credits);
            }
            println!("Total credits: {}", response.objective_value as i64);
            // Check geneds
            match crate::geneds::are_geneds_satisfied(&sched) {
                Ok(true) => println!("All GenEds satisfied!"),
                Ok(false) => println!("GenEd requirements NOT satisfied!"),
                Err(e) => println!("GenEd check error: {}", e),
            }
        }
        _ => {
            println!(
                "No feasible solution found. Status: {:?}",
                response.status()
            );
        }
    }
}
