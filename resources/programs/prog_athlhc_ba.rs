#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Athletic Health Care".to_string(),
        semesters: vec![
            vec![CC!("EXSC", 2210), CC!("EXSC", 2263)],
            vec![CC!("EXSC", 2209)],
            vec![
                CC!("ATHC", 3374),
                CC!("ATHC", 2212),
                CC!("BIOL", 2242),
                CC!("ATHC", 2325),
            ],
            vec![
                CC!("ATHC", 3364),
                CC!("ATHC", 3362),
                CC!("ATHC", 2213),
                CC!("BIOL", 2243),
                CC!("EXSC", 3303),
            ],
            vec![CC!("EXSC", 3357), CC!("EXSC", 3380), CC!("ATHC", 3312)],
            vec![CC!("ATHC", 3361), CC!("EXSC", 3366), CC!("ATHC", 3313)],
            vec![CC!("ATHC", 4406), CC!("ATHC", 4407), CC!("ATHC", 4412)],
            vec![CC!("ATHC", 4413), CC!("ATHC", "COMP")],
        ],
        assoc_stems: vec!["ATHC".to_string(), "EXSC".to_string()],
        electives: vec![],
    }
}
// Elective info: Almost none of the foundations are covered by this major
