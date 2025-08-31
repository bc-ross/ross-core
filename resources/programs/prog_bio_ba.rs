#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Biology".to_string(),
        semesters: vec![
            vec![CC!("CHEM", 1200), CC!("CHEM", 1201), CC!("BIOL", 1121)],
            vec![CC!("CHEM", 1210), CC!("CHEM", 1211), CC!("BIOL", 1122)],
            vec![CC!("CHEM", 2200), CC!("CHEM", 2201), CC!("BIOL", 3310)],
            vec![CC!("CHEM", 2210), CC!("CHEM", 2211), CC!("BIOL", 3305)],
            vec![CC!("XD", "XP")],
            vec![CC!("XD", "XP")],
            vec![CC!("XD", "XP")],
            vec![CC!("BIOL", "COMP")],
        ],
        assoc_stems: vec!["BIOL".to_string(), "CHEM".to_string()],
        electives: vec![],
    }
}
// Elective info: Electives for bio_ba: MATH-1300 or MATH-1250, PHYS-2000+PHYS-2001 or PHYS-2100+PHYS-2101, PHYS-2010+PHYS-2011 or PHYS-2110+PHYS-2111, & 5 BIOL+labs above BIOL-3311
