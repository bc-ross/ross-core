#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA English & Theatre Arts".to_string(),
        semesters: vec![
            vec![CC!("THTR", 1010), CC!("THTR", 1550)],
            vec![CC!("THTR", 1150), CC!("ENGL", 1600)],
            vec![CC!("THTR", 2800), CC!("THTR", 2210), CC!("ENGL", 1650)],
            vec![CC!("THTR", 2245), CC!("ENGL", 1700)],
            vec![CC!("THTR", 4150)],
            vec![CC!("ENGL", 1750), CC!("THTR", 3800)],
            vec![CC!("ENGL", 4310), CC!("THTR", "COMP")],
            vec![CC!("ENGL", 4110), CC!("ENGL", "COMP")],
        ],
        assoc_stems: vec!["THTR".to_string(), "ENGL".to_string()],
        electives: vec![],
    }
}
// Elective info: ENGL-3020 or THTR-3020 ; ENGL-1500 or ENGL-1550 ; 2 from THTR-3810, THTR-3820, THTR-3830 ; THTR-3560 or THTR-3580 ; 6 semesters of production arts
