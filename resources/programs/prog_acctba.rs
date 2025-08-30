#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Accounting".to_string(),
        semesters: vec![
            vec![CC!("ACCT", 2090), CC!("MGMT", 2250)],
            vec![CC!("ACCT", 2100)],
            vec![
                CC!("ACCT", 3270),
                CC!("ACCT", 3730),
                CC!("ECON", 2100),
                CC!("FINC", 3100),
            ],
            vec![CC!("ACCT", 3280), CC!("ACCT", 3820), CC!("ECON", 2090)],
            vec![
                CC!("BUSI", 3710),
                CC!("MKTG", 3100),
                CC!("ACCT", 3630),
                CC!("ACCT", 4010),
            ],
            vec![CC!("ACCT", 3640)],
            vec![CC!("ACCT", 4200), CC!("ACCT", 4930), CC!("BUSI", 4850)],
            vec![CC!("BUSI", 4860), CC!("BUSI", 4900)],
        ],
        assoc_stems: vec!["ACCT".to_string(), "BUSI".to_string()],
        electives: vec![],
    }
}
// Must take BUSI-2650 or MATH-1220 as an elective option
