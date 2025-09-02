#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA History".to_string(),
        semesters: vec![
            vec![CC!("HIST", 1100)],
            vec![CC!("HIST", 1101)],
            vec![CC!("HIST", 1300), CC!("HIST", 2000)],
            vec![CC!("HIST", 1380)],
            vec![],
            vec![],
            vec![],
            vec![CC!("HIST", 4000), CC!("HIST", "COMP")],
        ],
        assoc_stems: vec!["HIST".to_string()],
        electives: vec![],
    }
}
// Elective info: Upper division courses in: anct or med; modern Eu; US; non-west; 2 elecs
