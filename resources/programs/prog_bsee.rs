#![allow(unused_imports)]

use crate::schedule::{CourseCode, Elective::*, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "Electrical Engineering".to_string(),
        semesters: vec![
            vec![CC!("EENG", 2060), CC!("EENG", 2070)],
            vec![CC!("ENGR", 3710)],
            vec![CC!("ENGR", 4400)],
        ],
        assoc_stems: vec!["ENGR".to_string(), "EENG".to_string()],
        electives: vec![],
    }
}
