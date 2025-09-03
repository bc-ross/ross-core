#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BS Electrical Engineering".to_string(),
        semesters: vec![
            vec![
                CC!("ENGR", 1200),
                CC!("CHEM", 1200),
                CC!("CHEM", 1201),
                CC!("MATH", 1300),
                CC!("PHYS", 2100),
                CC!("PHYS", 2101),
            ],
            vec![
                CC!("MATH", 1350),
                CC!("PHYS", 2110),
                CC!("PHYS", 2111),
                CC!("EENG", 2010),
                CC!("EENG", 2020),
            ],
            vec![
                CC!("EENG", 2060),
                CC!("EENG", 3060),
                CC!("ENGR", 3150),
                CC!("MATH", 2300),
                CC!("THEO", 2000),
            ],
            vec![
                CC!("EENG", 3130),
                CC!("EENG", 4520),
                CC!("EENG", 4530),
                CC!("CSCI", 2300),
                CC!("MATH", 2500),
                CC!("MATH", 3100),
            ],
            vec![
                CC!("EENG", 3140),
                CC!("EENG", 3160),
                CC!("EENG", 3210),
                CC!("EENG", 3080),
                CC!("PHIL", 3250),
            ],
            vec![
                CC!("EENG", 4050),
                CC!("EENG", 4060),
                CC!("EENG", 4090),
                CC!("EENG", 4210),
                CC!("EENG", 4220),
            ],
            vec![CC!("EENG", 4600), CC!("ENGR", 3170)],
            vec![CC!("EENG", 4610)],
            vec![CC!("ENGR", "COMP")],
        ],
        assoc_stems: vec!["ENGR".to_string(), "EENG".to_string()],
        electives: vec![],
    }
}
// Elective info: can take EENG-4010+EENG-4020 or EENG-4510 :: 12 credits of technical electives :: 3 credits of math or sci electives
