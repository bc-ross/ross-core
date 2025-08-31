#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Chemistry".to_string(),
        semesters: vec![
            vec![CC!("CHEM", 1200), CC!("CHEM", 1201), CC!("MATH", 1300)],
            vec![CC!("CHEM", 1210), CC!("CHEM", 1211), CC!("MATH", 1350)],
            vec![
                CC!("CHEM", 2200),
                CC!("CHEM", 2201),
                CC!("PHYS", 2100),
                CC!("PHYS", 2101),
            ],
            vec![
                CC!("CHEM", 2210),
                CC!("CHEM", 2211),
                CC!("PHYS", 2110),
                CC!("PHYS", 2111),
            ],
            vec![
                CC!("CHEM", 3300),
                CC!("CHEM", 3301),
                CC!("CHEM", 3500),
                CC!("CHEM", 3501),
                CC!("CHEM", 4900),
            ],
            vec![
                CC!("CHEM", 3311),
                CC!("CHEM", 3400),
                CC!("CHEM", 3401),
                CC!("CHEM", 3800),
                CC!("CHEM", 3801),
                CC!("CHEM", 4901),
            ],
            vec![CC!("CHEM", 4902)],
            vec![CC!("CHEM", 4903), CC!("CHEM", "COMP")],
        ],
        assoc_stems: vec!["CHEM".to_string()],
        electives: vec![],
    }
}
// Elective info: two from: CHEM-3150, CHEM-3250, CHEM-3510, CHEM-3650, CHEM-3980, CHEM-4980, CHEM-4350, CHEM-4450, CHEM-4650, CHEM-4200
