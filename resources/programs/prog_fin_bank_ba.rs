#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Finance (Banking)".to_string(),
        semesters: vec![
            vec![CC!("ACCT", 2090), CC!("MGMT", 2250)],
            vec![CC!("ACCT", 2100)],
            vec![
                CC!("FINC", 3100),
                CC!("MKTG", 3100),
                CC!("ECON", 2090),
                CC!("THEO", 2000),
            ],
            vec![CC!("BUSI", 3710), CC!("ECON", 2100), CC!("PHIL", 3250)],
            vec![CC!("FINC", 3300), CC!("FINC", 4100)],
            vec![CC!("ECON", 3060), CC!("FINC", 4300), CC!("FINC", 4910)],
            vec![CC!("BUSI", 4850), CC!("FINC", 4330)],
            vec![
                CC!("BUSI", 4900),
                CC!("BUSI", 4860),
                CC!("FINC", 3950),
                CC!("FINC", "COMP"),
            ],
        ],
        assoc_stems: vec!["BUSI".to_string(), "FINC".to_string()],
        electives: vec![],
    }
}
// Elective info: 2 elecs from ACCT-4200, ECON-4130, FINC-4650, FINC-4940 ; BUSI-2650 or MATH-1220
