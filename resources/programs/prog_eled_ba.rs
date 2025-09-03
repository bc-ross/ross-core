#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Elementary Education".to_string(),
        semesters: vec![
            vec![CC!("EDUC", 2200), CC!("EDUC", 2201)],
            vec![CC!("EDUC", 2222)],
            vec![CC!("EDUC", 2214), CC!("EDUC", 2220), CC!("EXSC", 3302)],
            vec![],
            vec![CC!("EDUC", 3312), CC!("EDUC", 3313)],
            vec![
                CC!("EDUC", 3314),
                CC!("EDUC", 3303),
                CC!("EDUC", 3315),
                CC!("EDUC", 3309),
            ],
            vec![
                CC!("EDUC", 3301),
                CC!("EDUC", 3319),
                CC!("EDUC", 4451),
                CC!("EDUC", 3318),
                CC!("EDUC", 4462),
                CC!("EDUC", 4455),
            ],
            vec![
                CC!("EDUC", 4470),
                CC!("EDUC", 4492),
                CC!("EDUC", "DPROF"),
                CC!("EDUC", "COMP"),
            ],
        ],
        assoc_stems: vec!["EDUC".to_string()],
        electives: vec![],
    }
}
// Elective info: need to take an area of concetration (minor in smtg or SpEd)
