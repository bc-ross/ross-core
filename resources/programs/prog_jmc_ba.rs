#![allow(unused_imports)]

use crate::CC;
use crate::CC;
use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};

pub fn prog() -> Program {
    Program {
        name: "BA Journalism & Mass Communications".to_string(),
        semesters: vec![
            vec![CC!("MCOM", 1000), CC!("MCOM", 1500)],
            vec![CC!("MCOM", 1610), CC!("MCOM", 2000)],
            vec![CC!("MCOM", 2610), CC!("JOUR", 2620)],
            vec![CC!("JOUR", 3300)],
            vec![CC!("JOUR", 3350)],
            vec![],
            vec![],
            vec![CC!("MCOM", 4090), CC!("MCOM", "COMP")],
        ],
        assoc_stems: vec!["MCOM".to_string(), "JOUR".to_string()],
        electives: vec![],
    }
}
// Elective info:
