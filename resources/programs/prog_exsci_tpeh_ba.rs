#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Exercise Science (Teaching Physical Education & Health)".to_string(),
        semesters: vec![
            vec![CC!("EXSC", 1150), CC!("EXSC", 2209)],
            vec![CC!("EXSC", 2210), CC!("EXSC", 2263)],
            vec![CC!("EXSC", 2220), CC!("BIOL", 2242)],
            vec![CC!("BIOL", 2243), CC!("EXSC", 2222), CC!("EXSC", 3357)],
            vec![CC!("EXSC", 3366), CC!("EXSC", 3350)],
            vec![CC!("EXSC", 3380), CC!("EXSC", 3365), CC!("EXSC", 3302)],
            vec![CC!("EXSC", 4457)],
            vec![CC!("EXSC", "SWMP"), CC!("EXSC", "COMP")],
        ],
        assoc_stems: vec!["EXSC".to_string()],
        electives: vec![],
    }
}
// Elective info: EXSC-1101 or EXSC-1123 or EXSC-1126 or EXSC-1111 ; EXSC-4402 or ATHC-4406 ;  EXSC-3303 and'or EXSC-3340 ; fyi, I ignored the teaching liscensure reqs
