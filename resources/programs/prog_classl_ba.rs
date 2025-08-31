#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Classics (Latin)".to_string(),
        semesters: vec![
            vec![CC!("LATN", 1000), CC!("GRBK", 1750)],
            vec![CC!("LATN", 1020)],
            vec![CC!("GREK", 1000)],
            vec![CC!("GREK", 1020)],
            vec![],
            vec![],
            vec![],
            vec![CC!("CLSC", "COMP")],
        ],
        assoc_stems: vec!["LATN".to_string(), "CLSC".to_string()],
        electives: vec![],
    }
}
// Elective info: must take HIST-3520 or HIST-3521 :: 2 out of= ARCH-2300, ART-3411, ENGL-3060, GRBK-2750, HIST-3520, HIST-3521, HIST-3522, HIST-3541, PHIL-4010, PHIL-4020, THEO-3420, THTR-3810 :: 6 LATN-3000+
