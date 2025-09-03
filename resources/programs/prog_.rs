#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "".to_string(),
        semesters: vec![vec![]],
        assoc_stems: vec![],
        electives: vec![],
    }
}
// Elective info:
