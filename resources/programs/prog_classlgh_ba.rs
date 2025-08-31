#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "Classics BA (Latin, Greek, & Hebrew)".to_string(),
        semesters: vec![
            vec![CC!("LATN", 1000), CC!("GRBK", 1750)],
            vec![CC!("LATN", 1020)],
            vec![CC!("GREK", 1000)],
            vec![CC!("GREK", 1020)],
            vec![CC!("THEO", 2010)],
            vec![CC!("THEO", 2020)],
            vec![],
            vec![CC!("CLSC", "COMP")],
        ],
        assoc_stems: vec!["GREK".to_string(), "LATN".to_string(), "CLSC".to_string()],
        electives: vec![],
    }
}
// Elective info: must take HIST-3520 or HIST-3521 :: 2 out of= ARCH-2300, ART-3411, ENGL-3060, GRBK-2750, HIST-3520, HIST-3521, HIST-3522, HIST-3541, PHIL-4010, PHIL-4020, THEO-3420, THTR-3810 :: 4 total classes of LATN-3000+ and GREK-3000+ :: THEO-2010 usually offered in even fall, but it is at the discretion of the Theo department, so double check
