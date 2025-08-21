mod model;
mod model_context;
mod model_courses;
mod model_geneds;
mod model_prereqs;
mod model_semester;
mod two_stage_schedule;

#[path = "../../src/geneds.rs"]
mod geneds;
#[path = "../../src/load_catalogs.rs"]
mod load_catalogs;
#[path = "../../src/prereqs.rs"]
mod prereqs;
#[path = "../../src/schedule.rs"]
mod schedule;

use crate::model::{ModelBuilderContext, build_model_pipeline};
use cp_sat::proto::CpSolverStatus;
use schedule::CourseCode;

fn main() {
    // --- Use a real catalog and fake schedule ---
    use crate::load_catalogs::CATALOGS;
    let catalog = &CATALOGS[0];
    let mut course_codes: Vec<CourseCode> = catalog.courses.keys().cloned().collect();
    // Prepare 8 semesters, plus semester 0 for incoming courses
    let mut semesters: Vec<Vec<CourseCode>> = vec![vec![]; 8];
    // Assign first 8 courses to semesters 1..8 (not semester 0)
    let assigned: Vec<CourseCode> = course_codes.drain(0..8).collect();
    for (i, code) in assigned.into_iter().enumerate() {
        semesters[i].push(code); // i from 0..7, so semesters[0..7] (which are semesters 1..8)
    }
    // Add additional courses to semester 1 (index 0)
    semesters[0].push(CC!("PSYC", 2731)); // Foundation
    semesters[0].push(CC!("HONR", 1030)); // Core
    semesters[0].push(CC!("THEO", 3820)); // Foundation & S&P
    semesters[0].push(CC!("THEO", 2000)); // Foundation
    semesters[0].push(CC!("THEO", 2100)); // Foundation
    semesters[0].push(CC!("THEO", 3100)); // Foundation
    semesters[0].push(CC!("THEO", 3110)); // Foundation
    // Incoming courses are tracked separately, not in the main semester list
    let incoming_courses = vec![CC!("CHEM", 2200)]; // Example incoming course
    // Only semesters 1-8 in courses, incoming courses in incoming
    let sched = schedule::Schedule {
        courses: semesters.clone(),
        programs: vec![],
        incoming: incoming_courses.clone(),
        catalog: catalog.clone(),
    };
    println!("{:?}", &sched.courses);
    let max_credits_per_semester = 18;
    // Call the new two-stage scheduling function, mutably updating sched.courses
    let mut sched = sched;
    crate::two_stage_schedule::two_stage_lex_schedule(&mut sched, max_credits_per_semester)
        .unwrap();
    // Print the updated schedule from sched.courses
    println!("Final schedule (two-stage, balanced):");
    let mut sched_credits = 0;
    for (s, semester) in std::iter::once(&sched.incoming)
        .chain(sched.courses.iter())
        .enumerate()
    {
        if s == 0 {
            println!("Semester 0 (incoming only):");
        } else {
            println!("Semester {}", s);
        }
        let mut sem_credits = 0;
        for code in semester {
            // Look up credits from catalog
            let credits = sched
                .catalog
                .courses
                .get(code)
                .and_then(|(_, cr, _)| *cr)
                .unwrap_or(0);
            println!("  {} ({} credits)", code, credits);
            sem_credits += credits;
        }
        println!("  Credits: {}", sem_credits);
        if s > 0 {
            sched_credits += sem_credits;
        }
    }
    println!("Total credits (excluding incoming): {}", sched_credits);
    match crate::geneds::are_geneds_satisfied(&sched) {
        Ok(true) => println!("All GenEds satisfied!"),
        Ok(false) => println!("GenEd requirements NOT satisfied!"),
        Err(e) => println!("GenEd check error: {}", e),
    }
}
