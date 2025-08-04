#![allow(unused_imports)]

use crate::schedule::{CourseCode, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "Example #4".to_string(),
        semesters: vec![
            vec![CC!("ASTR", 1010), CC!("ASTR", 1030)],
            vec![CC!("ASTR", 1040), CC!("ASTR", 1050)],
        ],
        electives: vec![],
        assoc_stems: vec!["ASTR".to_string()],
    }
}
