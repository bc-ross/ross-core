#![allow(unused_imports)]

use crate::schedule::{CourseCode, Elective::*, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "Architecture".to_string(),
        semesters: vec![
            vec![CC!("ARCH", 1060), CC!("ARCH", 2000)],
            vec![CC!("ARCH", 2600), CC!("ARCH", 2760)],
        ],
        assoc_stems: vec!["ARCH".to_string()],
        electives: vec![],
    }
}
