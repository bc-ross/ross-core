#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BS Computer Science".to_string(),
        semesters: vec![
            vec![],
            vec![CC!("MATH", 1220)],
            vec![CC!("CSCI", 1140), CC!("MATH", 2550)],
            vec![CC!("CSCI", 2150), CC!("CSCI", 2560)],
            vec![CC!("CSCI", 3100), CC!("CSCI", 3570)],
            vec![CC!("CSCI", 3500), CC!("MATH", 3400)],
            vec![CC!("CSCI", 3600), CC!("CSCI", 4200), CC!("CSCI", 4920)],
            vec![CC!("CSCI", 4400), CC!("CSCI", 4930), CC!("CSCI", "COMP")],
        ],
        assoc_stems: vec!["CSCI".to_string()],
        electives: vec![],
    }
}
// Elective info: 6 credits of CSCI electives (at least 1 must be CSCI-3000+)
