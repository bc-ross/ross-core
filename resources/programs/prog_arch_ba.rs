#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Architecture".to_string(),
        semesters: vec![
            vec![CC!("ARCH", 1300), CC!("ART", 1000), CC!("ART", 1010)],
            vec![CC!("ARCH", 1200), CC!("ARCH", 1410)],
            vec![CC!("ARCH", 2111), CC!("ARCH", 2201), CC!("ARCH", 2300)],
            vec![CC!("ARCH", 2112), CC!("ARCH", 2301)],
            vec![CC!("ARCH", 3113), CC!("ARCH", 3400), CC!("ENGR", 2300)],
            vec![CC!("ARCH", 3114), CC!("ENGR", 2320)],
            vec![CC!("ARCH", 4115), CC!("ARCH", 4400)],
            vec![CC!("ARCH", 4116), CC!("ARCH", "COMP"), CC!("CIVL", 3550)],
        ],
        assoc_stems: vec!["ARCH".to_string()],
        electives: vec![],
    }
}
// Elective info:
