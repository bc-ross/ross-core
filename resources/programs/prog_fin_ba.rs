#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Finance".to_string(),
        semesters: vec![
            vec![CC!("ACCT", 2090), CC!("MGMT", 2250)],
            vec![CC!("ACCT", 2100)],
            vec![CC!("ECON", 2090)],
            vec![CC!("ECON", 2100), CC!("FINC", 3100)],
            vec![
                CC!("THEO", 2000),
                CC!("MKTG", 3100),
                CC!("BUSI", 3710),
                CC!("FINC", 4100),
            ],
            vec![CC!("ECON", 3060), CC!("FINC", 4650), CC!("FINC", 4910)],
            vec![
                CC!("PHIL", 3250),
                CC!("ACCT", 4200),
                CC!("BUSI", 4850),
                CC!("FINC", 4900),
            ],
            vec![
                CC!("BUSI", 4900),
                CC!("BUSI", 4860),
                CC!("FINC", 4950),
                CC!("FINC", "COMP"),
            ],
        ],
        assoc_stems: vec!["BUSI".to_string(), "FINC".to_string()],
        electives: vec![],
    }
}
// Elective info: BUSI-2650 or MATH-1220 ; ECON or ACCT 3000+ elec ; FINC 3000+ elec
