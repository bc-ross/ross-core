#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Exercise Science (General Health Care)".to_string(),
        semesters: vec![
            vec![CC!("EXSC", 1150), CC!("EXSC", 2263), CC!("PSYC", 1000)],
            vec![CC!("EXSC", 2209), CC!("EXSC", 2210)],
            vec![CC!("BIOL", 2242), CC!("EXSC", 3303)],
            vec![CC!("BIOL", 2243), CC!("SOCI", 1000), CC!("ATHC", 2325)],
            vec![CC!("EXSC", 3357), CC!("EXSC", 3380)],
            vec![CC!("EXSC", 3366), CC!("EXSC", 3303)],
            vec![CC!("ATHC", 4407), CC!("EXSC", "COMP")],
            vec![CC!("EXSC", 4790)],
        ],
        assoc_stems: vec!["EXSC".to_string(), "ATHC".to_string()],
        electives: vec![],
    }
}
// Elective info: BIOL-1107 or BIOL-1121+BIOL-1122 ; (CHEM-1010+CHEM-1011) or (CHEM-1200+CHEM-1201 & CHEM-1210+CHEM-1211) ; EXSC-4402 or ATHC-4406 ; 1 PSYC elec ; 1 SOCI elec ; PHIL-3250 or THEO-3940
