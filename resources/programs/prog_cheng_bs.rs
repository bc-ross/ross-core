#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BS Chemical Engineering".to_string(),
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
                CC!("CHEM", 1210),
                CC!("CHEM", 1211),
                CC!("MATH", 1350),
                CC!("PHYS", 2110),
                CC!("PHYS", 2111),
            ],
            vec![
                CC!("CENG", 2010),
                CC!("CHEM", 2200),
                CC!("CHEM", 2201),
                CC!("MATH", 2300),
                CC!("EENG", 2060),
            ],
            vec![
                CC!("THEO", 2000),
                CC!("CENG", 3300),
                CC!("ENGR", 3150),
                CC!("ENGR", 3250),
                CC!("CHEM", 2210),
                CC!("MATH", 3100),
            ],
            vec![
                CC!("CENG", 3350),
                CC!("CENG", 3250),
                CC!("ENGR", 3170),
                CC!("ENGR", 3500),
            ],
            vec![
                CC!("CENG", 3050),
                CC!("CENG", 4210),
                CC!("ENGR", 3410),
                CC!("ENGR", 3600),
                CC!("PHIL", 3250),
            ],
            vec![CC!("CENG", 4600), CC!("CENG", 4080), CC!("CENG", 4350)],
            vec![CC!("CENG", 4610), CC!("CENG", 4820)],
            vec![CC!("ENGR", 4840), CC!("CENG", "COMP")],
        ],
        assoc_stems: vec!["ENGR".to_string(), "CENG".to_string()],
        electives: vec![],
    }
}
// Elective info: one upper level chem, 2 ceng electives
