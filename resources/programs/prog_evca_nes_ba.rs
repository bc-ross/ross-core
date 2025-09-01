#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Evangelization & Catechesis (New Evangelization Studies)".to_string(),
        semesters: vec![
            vec![CC!("THEO", 1100)],
            vec![],
            vec![CC!("EVCA", 2100)],
            vec![CC!("THEO", 2000), CC!("EVCA", 2150)],
            vec![CC!("THEO", 3280)],
            vec![],
            vec![CC!("THEO", 3640)],
            vec![CC!("EVCA", 4500), CC!("EVCA", 4700), CC!("EVCA", "COMP")],
        ],
        assoc_stems: vec!["EVCA".to_string(), "THEO".to_string()],
        electives: vec![],
    }
}
// Elective info: THEO-2100 or THEO-3100 or THEO-3110 ; THEO-2150 or THEO-3160 ; THEO-3920 or THEO-3430 ; 2 courses out of EVCA-3200, EVCA-3300, EVCA-3400, THEO-3960 ; one more EVCA 3 credit course (or THEO-3960)
