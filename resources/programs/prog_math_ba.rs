#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Mathematics".to_string(),
        semesters: vec![
            vec![CC!("MATH", 1300)],
            vec![CC!("MATH", 1350)],
            vec![CC!("MATH", 2300)],
            vec![CC!("MATH", 2500)],
            vec![CC!("MATH", 3600), CC!("MATH", 3200)],
            vec![],
            vec![CC!("MATH", 4930)],
            vec![CC!("MATH", "COMP")],
        ],
        assoc_stems: vec!["MATH".to_string()],
        electives: vec![],
    }
}
// Elective info: CSCI-1140 or CSCI-2300; MATH-3610 or MATH-4700 or MATH-4800
