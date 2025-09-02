#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA French".to_string(),
        semesters: vec![
            vec![CC!("FREN", 1000)],
            vec![CC!("FREN", 1020)],
            vec![CC!("FREN", 2010)],
            vec![CC!("FREN", 3040)],
            vec![CC!("FREN", 3610)],
            vec![],
            vec![],
            vec![CC!("FREN", "COMP")],
        ],
        assoc_stems: vec!["FREN".to_string()],
        electives: vec![],
    }
}
// Elective info: FREN-3700 or FREN-3650 ; 15 credits of FREN-3000+
