#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Marketing".to_string(),
        semesters: vec![
            vec![CC!("MGMT", 2250)],
            vec![],
            vec![CC!("ACCT", 2090), CC!("ECON", 2090)],
            vec![CC!("ACCT", 2100), CC!("ECON", 2100), CC!("MKTG", 3100)],
            vec![CC!("FINC", 3100), CC!("MKTG", 3880)],
            vec![CC!("BUSI", 3710), CC!("MKTG", 3810)],
            vec![CC!("BUSI", 4850), CC!("MKTG", 4830)],
            vec![
                CC!("BUSI", 4900),
                CC!("BUSI", 4860),
                CC!("MKTG", 4850),
                CC!("MKTG", "COMP"),
            ],
        ],
        assoc_stems: vec!["BUSI".to_string(), "MKTG".to_string()],
        electives: vec![],
    }
}
// Elective info: BUSI-2650 or MATH-1220
