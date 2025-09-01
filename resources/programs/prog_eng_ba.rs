#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA English".to_string(),
        semesters: vec![
            vec![],
            vec![CC!("ENGL", 1600)],
            vec![CC!("ENGL", 1650), CC!("ENGL", 1700)],
            vec![CC!("ENGL", 1750)],
            vec![CC!("ENGL", 4310)],
            vec![],
            vec![CC!("ENGL", 4110)],
            vec![CC!("ENGL", 4910), CC!("ENGL", "COMP")],
        ],
        assoc_stems: vec!["ENGL".to_string()],
        electives: vec![],
    }
}
// Elective info: must take one out of: ENGL-1500, ENGL-1550, ENGL-1575 :: Take 9 credits of upper division courses, at least one from both Genre Courses and Literary Period Courses
