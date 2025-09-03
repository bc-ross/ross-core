#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Computer Science".to_string(),
        semesters: vec![
            vec![],
            vec![],
            vec![CC!("CSCI", 1140), CC!("MATH", 2550)],
            vec![CC!("CSCI", 2150), CC!("CSCI", 2560)],
            vec![CC!("CSCI", 3100)],
            vec![CC!("CSCI", 3500)],
            vec![CC!("CSCI", 4200), CC!("CSCI", 4920)],
            vec![CC!("CSCI", 4400), CC!("CSCI", 4930), CC!("CSCI", "COMP")],
        ],
        assoc_stems: vec!["CSCI".to_string()],
        electives: vec![],
    }
}
// Elective info: must take MATH-1220 or MATH-1300
