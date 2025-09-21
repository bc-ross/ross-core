#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BS Mechanical Engineering".to_string(),
        semesters: vec![
            vec![
                CC!("ENGR", 1200),
                CC!("PHYS", 2100),
                CC!("PHYS", 2101),
                CC!("CHEM", 1200),
                CC!("CHEM", 1201),
                CC!("MATH", 1300),
            ],
            vec![
                CC!("ENGR", 1500),
                CC!("ENGR", 1520),
                CC!("PHYS", 2110),
                CC!("PHYS", 2111),
                CC!("MATH", 1350),
            ],
            vec![CC!("ENGR", 2300), CC!("ENGR", 3500), CC!("MATH", 2300)],
            vec![
                CC!("ENGR", 2310),
                CC!("ENGR", 2320),
                CC!("MATH", 3100),
                CC!("ENGR", 3250),
                CC!("MENG", 3180),
                CC!("THEO", 2000),
            ],
            vec![
                CC!("MENG", 3220),
                CC!("ENGR", 3300),
                CC!("ENGR", 3400),
                CC!("PHIL", 3250),
            ],
            vec![
                CC!("MENG", 3240),
                CC!("MENG", 4240),
                CC!("ENGR", 3600),
                CC!("ENGR", 3150),
            ],
            vec![
                CC!("MENG", 4600),
                CC!("ENGR", 3170),
                CC!("MENG", 4730),
                CC!("MENG", 4700),
            ],
            vec![CC!("MENG", 4610), CC!("ENGR", 3410)],
            vec![CC!("MENG", "COMP")],
        ],
        assoc_stems: vec!["ENGR".to_string(), "MENG".to_string()],
        electives: vec![],
    }
}
// Elective info: ENGR-2000 or CSCI-2300; EENG-2060+EENG-3060 or PHYS-3500
