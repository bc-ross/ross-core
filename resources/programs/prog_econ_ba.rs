#![allow(unused_imports)]

use crate::geneds::ElectiveReq::*;
use crate::schedule::{CourseCode, Elective, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "BA Economics".to_string(),
        semesters: vec![
            vec![CC!("ACCT", 2090)],
            vec![],
            vec![CC!("ECON", 2100)],
            vec![CC!("ECON", 2090)],
            vec![CC!("ECON", 3000), CC!("ECON", 3100), CC!("MATH", 1220)],
            vec![CC!("ECON", 3090)],
            vec![CC!("ECON", 4130)],
            vec![CC!("ECON", 4110), CC!("ECON", "COMP")],
        ],
        assoc_stems: vec!["ECON".to_string()],
        electives: vec![],
    }
}
// Elective info: 3 courses out of= ECON-3060, ECON-3120, ECON-3150, ECON-3200, ECON-3260, ECON-3980, ECON-4000, ECON-4010, ECON-4030, ECON-4160, ECON-4990
