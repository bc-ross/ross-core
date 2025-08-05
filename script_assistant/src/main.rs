use std::fs::File;
use std::io::Write;

#[path = "../../resources/course_reqs/mod.rs"]
mod course_reqs;

#[path = "../../resources/programs/mod.rs"]
mod programs;

#[path = "../../resources/credits.rs"]
mod credits;

#[path = "../../src/schedule.rs"]
pub mod schedule;

#[path = "../../src/prereqs.rs"]
pub mod prereqs;

fn main() {
    let mut courses = vec![];
    for program in programs::programs() {
        courses.extend(program.semesters.into_iter().flatten());
    }
    for (course, prereq) in course_reqs::prereqs() {
        courses.push(course);
        courses.extend(prereq.all_course_codes());
    }
    // Implement geneds later

    let courses_json = serde_json::to_string_pretty(&courses).unwrap();
    let mut file = File::create("courses.json").unwrap();
    file.write_all(courses_json.as_bytes()).unwrap();
    println!("Exported {} courses", courses.len());
}
