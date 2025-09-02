#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Exercise Science (Sports Management)".to_string(),
        semesters: vec![
            vec![CC!("EXSC", 1150), CC!("EXSC", 2209)],
            vec![CC!("EXSC", 2210), CC!("EXSC", 2263), CC!("ACCT", 2090)],
            vec![CC!("ECON", 2090), CC!("EXSC", 3357)],
            vec![CC!("ECON", 2100)],
            vec![CC!("ACCT", 2100), CC!("MGMT", 2250), CC!("EXSC", 3380)],
            vec![CC!("FINC", 3100), CC!("MKTG", 3100)],
            vec![CC!("EXSC", 3366), CC!("EXSC", "SWMP"), CC!("EXSC", 4411)],
            vec![CC!("EXSC", "COMP")],
        ],
        assoc_stems: vec!["EXSC".to_string(), "ATHC".to_string()],
        electives: vec![],
    }
}
// Elective info: EXSC-1101 or EXSC-1123 or EXSC-1126 or EXSC-1111 ; EXSC-4402 or ATHC-4406 ; EXSC-2240 or BIOL-2242+BIOL-2243 ; EXSC-3303 and'or EXSC-3340
