#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Graphic Design".to_string(),
        semesters: vec![
            vec![CC!("ART", 1000), CC!("ART", 1010)],
            vec![CC!("ART", 1030), CC!("ART", 2800)],
            vec![CC!("ART", 2300), CC!("ART", 2500)],
            vec![CC!("ART", 3301), CC!("ART", 3412)],
            vec![CC!("ART", 3415), CC!("ART", 3310)],
            vec![CC!("ART", 3413), CC!("MKTG", 3100), CC!("ART", 3302)],
            vec![CC!("ART", 4310), CC!("ART", 4311)],
            vec![CC!("ART", 4950), CC!("JOUR", 4750)],
        ],
        assoc_stems: vec!["ART".to_string()],
        electives: vec![],
    }
}
// Elective info: 2 graphic design elecs
