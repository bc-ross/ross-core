#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BS Engineering Physics".to_string(),
        semesters: vec![
            vec![
                CC!("ENGR", 1200),
                CC!("MATH", 1300),
                CC!("PHYS", 2100),
                CC!("PHYS", 2101),
                CC!("CHEM", 1200),
                CC!("CHEM", 1201),
            ],
            vec![
                CC!("ENGR", 1500),
                CC!("MATH", 1350),
                CC!("PHYS", 2110),
                CC!("PHYS", 2111),
            ],
            vec![
                CC!("PHYS", 3200),
                CC!("PHYS", 3201),
                CC!("ENGR", 2300),
                CC!("MATH", 2300),
            ],
            vec![CC!("MATH", 3100)],
            vec![CC!("ENGR", 3170), CC!("ENGR", 3300), CC!("PHYS", 4900)],
            vec![
                CC!("ENGR", 3600),
                CC!("ENGR", 3410),
                CC!("PHYS", 4901),
                CC!("PHYS", 4910),
            ],
            vec![CC!("PHYS", 4600), CC!("PHYS", 4902)],
            vec![CC!("PHYS", 4903), CC!("PHYS", "COMP")],
        ],
        assoc_stems: vec!["ENGR".to_string(), "PHYS".to_string()],
        electives: vec![],
    }
}
// Elective info: CHEM-1210+CHEM-1211 or tech elec : PHYS-4700+ENGR-3400 or ENGR-3500 : PHYS-3210+PHYS-3211+PHYS-3201 or tech elec : EENG-2060+EENG-3060 or PHYS-3500 : PHYS-4400 or ENGR-3250 : ENGR-2000 or CSCI-2300 : ENGR-2310 or PHYS-4100 :: 9 credits tech elecs : one course design elec : one inst elec
