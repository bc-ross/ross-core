#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Management".to_string(),
        semesters: vec![
            vec![CC!("ACCT", 2090), CC!("MGMT", 2250)],
            vec![CC!("ACCT", 2100), CC!("BUSI", 2650)],
            vec![
                CC!("MGMT", 3250),
                CC!("BUSI", 3710),
                CC!("MKTG", 3100),
                CC!("ECON", 2100),
            ],
            vec![
                CC!("MGMT", 3500),
                CC!("MGMT", 3660),
                CC!("ECON", 2090),
                CC!("FINC", 3100),
            ],
            vec![CC!("BUSI", 4850), CC!("MGMT", 4660)],
            vec![CC!("BUSI", 4860), CC!("MGMT", 4500)],
            vec![CC!("ECON", 3200), CC!("MGMT", 4560)],
            vec![CC!("BUSI", 4900), CC!("BUSI", 4860), CC!("MGMT", "COMP")],
        ],
        assoc_stems: vec!["BUSI".to_string(), "MGMT".to_string()],
        electives: vec![],
    }
}
// Elective info: BUSI-2650 or MATH-1220
