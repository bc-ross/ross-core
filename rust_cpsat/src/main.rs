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

    // Call the new two-stage scheduling function
    let schedule_result =
        crate::two_stage_schedule::two_stage_lex_schedule(&sched, max_credits_per_semester);

    match schedule_result {
        Some(final_schedule) => {
            println!("Final schedule (two-stage, balanced):");
            let mut sched_credits = 0;
            for (s, semester) in final_schedule.iter().enumerate() {
                println!("Semester {}", s + 1);
                let mut sem_credits = 0;
                for (code, credits) in semester {
                    println!("  {} ({} credits)", code, credits);
                    sem_credits += credits;
                }
                println!("  Credits: {}", sem_credits);
                sched_credits += sem_credits;
            }
            println!("Total credits: {}", sched_credits);
            // Check geneds
            match crate::geneds::are_geneds_satisfied(&sched) {
                Ok(true) => println!("All GenEds satisfied!"),
                Ok(false) => println!("GenEd requirements NOT satisfied!"),
                Err(e) => println!("GenEd check error: {}", e),
            }
        }
        None => {
            println!("No feasible schedule found.");
        }
    }
}
