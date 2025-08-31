#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Art".to_string(),
        semesters: vec![
            vec![CC!("ART", 1000), CC!("ART", 1010)],
            vec![CC!("ART", 1030), CC!("ART", 2110)],
            vec![CC!("ART", 2200), CC!("ART", 2300)],
            vec![CC!("ART", 3411)],
            vec![CC!("ART", 3412)],
            vec![CC!("ART", 3900)],
            vec![CC!("ART", 4900)],
            vec![CC!("ART", 4901)],
        ],
        assoc_stems: vec!["ART".to_string()],
        electives: vec![],
    }
}
// Elective info: Need another art hist course and 7 3credit studios
