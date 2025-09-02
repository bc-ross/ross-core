#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Exercise Science (Coaching)".to_string(),
        semesters: vec![
            vec![CC!("EXSC", 1150), CC!("EXSC", 2209)],
            vec![CC!("EXSC", 2210), CC!("EXSC", 2263)],
            vec![CC!("EXSC", 2220), CC!("EXSC", "SWMP")],
            vec![CC!("EXSC", 3357), CC!("EXSC", 2222)],
            vec![CC!("EXSC", 3350), CC!("EXSC", 2266)],
            vec![CC!("EXSC", 3380)],
            vec![],
            vec![CC!("EXSC", "COMP")],
        ],
        assoc_stems: vec!["EXSC".to_string()],
        electives: vec![],
    }
}
// Elective info: EXSC-1101 or EXSC-1123 or EXSC-1126 or EXSC-1111 ; EXSC-4402 or ATHC-4406 ; EXSC-2240 or BIOL-2242+BIOL-2243 ; EXSC-3303 and'or EXSC-3340 ; 2 theory of coaching courses
