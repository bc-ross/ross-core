#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Criminology".to_string(),
        semesters: vec![
            vec![CC!("CRIM", 1000)],
            vec![],
            vec![],
            vec![CC!("CRIM", 3100), CC!("MATH", 1220)],
            vec![CC!("SOCI", 3155)],
            vec![],
            vec![CC!("CRIM", 4790)],
            vec![CC!("CRIM", "COMP")],
        ],
        assoc_stems: vec!["CRIM".to_string(), "SOCI".to_string()],
        electives: vec![],
    }
}
// Elective info: basics: THEO-2000 or PHIL-3250 :: CRIM-3200 or SOCI-4175. Otherwise this one is super complicated and requires 15 credits of electives within the major, 15 credits of electives from allied majors, and then there are 2 options: choose from a set of specified minors, or 5 courses from another set list
