#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BS Biology".to_string(),
        semesters: vec![
            vec![CC!("CHEM", 1200), CC!("CHEM", 1201), CC!("BIOL", 1121)],
            vec![CC!("CHEM", 1210), CC!("CHEM", 1211), CC!("BIOL", 1122)],
            vec![CC!("CHEM", 2200), CC!("CHEM", 2201), CC!("BIOL", 3310)],
            vec![CC!("CHEM", 2210), CC!("CHEM", 2211), CC!("BIOL", 3305)],
            vec![],
            vec![],
            vec![],
            vec![CC!("BIOL", "COMP")],
        ],
        assoc_stems: vec!["BIOL".to_string()],
        electives: vec![],
    }
}
// Elective info: Electives for bio_ba: MATH-1300 or MATH-1250, PHYS-1100 or PHYS-2000+PHYS-2001 or PHYS-2100+PHYS-2101, & 6 BIOL+labs above BIOL-3311
