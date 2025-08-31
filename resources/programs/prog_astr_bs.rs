#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BS Astronomy".to_string(),
        semesters: vec![
            vec![
                CC!("PHYS", 2100),
                CC!("PHYS", 2101),
                CC!("ASTR", 1300),
                CC!("MATH", 1300),
                CC!("CHEM", 1200),
                CC!("CHEM", 1201),
            ],
            vec![
                CC!("PHYS", 2110),
                CC!("PHYS", 2111),
                CC!("ASTR", 1400),
                CC!("MATH", 1350),
                CC!("CHEM", 1210),
                CC!("CHEM", 1211),
            ],
            vec![
                CC!("ASTR", 3000),
                CC!("MATH", 2300),
                CC!("PHYS", 3200),
                CC!("PHYS", 3201),
            ],
            vec![CC!("MATH", 3100), CC!("PHYS", 3210), CC!("PHYS", 3211)],
            vec![CC!("ASTR", 4100), CC!("PHYS", 4100), CC!("PHYS", 4900)],
            vec![
                CC!("ASTR", 4300),
                CC!("PHYS", 4110),
                CC!("PHYS", 4300),
                CC!("PHYS", 4301),
                CC!("PHYS", 4901),
            ],
            vec![
                CC!("ASTR", 4200),
                CC!("PHYS", 4400),
                CC!("PHYS", 4600),
                CC!("PHYS", 4800),
                CC!("PHYS", 4902),
                CC!("PHYS", 4910),
            ],
            vec![CC!("PHYS", 4610), CC!("PHYS", 4903), CC!("ASTR", "COMP")],
        ],
        assoc_stems: vec!["ASTR".to_string(), "PHYS".to_string()],
        electives: vec![],
    }
}
// Elective info: One of these three CSCI-2300, CSCI-1140, ENGR-2000
