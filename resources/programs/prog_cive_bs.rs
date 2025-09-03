#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BS Civil Engineering".to_string(),
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
                CC!("PHYS", 2110),
                CC!("PHYS", 2111),
                CC!("MATH", 1350),
            ],
            vec![
                CC!("ENGR", 2300),
                CC!("CIVL", 2150),
                CC!("MATH", 2300),
                CC!("CIVL", 2000),
            ],
            vec![
                CC!("ENGR", 2310),
                CC!("ENGR", 2320),
                CC!("MATH", 3100),
                CC!("CIVL", 2310),
            ],
            vec![
                CC!("CIVL", 3510),
                CC!("CIVL", 3120),
                CC!("ENGR", 3150),
                CC!("ENGR", 3300),
                CC!("THEO", 2000),
            ],
            vec![
                CC!("CIVL", 3010),
                CC!("CIVL", 3230),
                CC!("CIVL", 3310),
                CC!("PHIL", 3250),
            ],
            vec![CC!("CIVL", 3020), CC!("ENGR", 3170)],
            vec![CC!("CIVL", 4600)],
            vec![CC!("CIVL", 4700), CC!("CIVL", "COMP")],
        ],
        assoc_stems: vec!["ENGR".to_string(), "CIVL".to_string()],
        electives: vec![],
    }
}
// Elective info: 3 credits engineering elec= CENG-2010, EENG-2060, EENG-3060, any ENG-3000+, any MENG-3000+ :: 18 credits technical elecs= any CIVL-3000+ courses :: science elec= any basic science course not required by the major
