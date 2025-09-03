#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Biochemistry".to_string(),
        semesters: vec![
            vec![
                CC!("BIOL", 1121),
                CC!("CHEM", 1200),
                CC!("CHEM", 1201),
                CC!("MATH", 1300),
            ],
            vec![
                CC!("BIOL", 1122),
                CC!("CHEM", 1210),
                CC!("CHEM", 1211),
                CC!("MATH", 1350),
            ],
            vec![
                CC!("CHEM", 2200),
                CC!("CHEM", 2201),
                CC!("PHYS", 2100),
                CC!("PHYS", 2101),
            ],
            vec![
                CC!("CHEM", 2210),
                CC!("CHEM", 2211),
                CC!("PHYS", 2010),
                CC!("PHYS", 2011),
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
                CC!("CHEM", 3510),
                CC!("CHEM", 3511),
                CC!("CHEM", 4901),
            ],
            vec![CC!("CHEM", 4450), CC!("CHEM", 4451), CC!("CHEM", 4902)],
            vec![CC!("CHEM", 4903), CC!("BIOC", "COMP")],
        ],
        assoc_stems: vec!["CHEM".to_string(), "BIOL".to_string(), "BIOC".to_string()],
        electives: vec![],
    }
}
// Elective info: Must take one of these: CHEM-3150, CHEM-3250, CHEM-3400, CHEM-3650, CHEM-3800, CHEM-3980, CHEM-4980, CHEM-4350, CHEM-4650
