#![allow(unused_imports)]

use crate::schedule::{CourseCode, Elective::*, Program};
use crate::CC;

pub fn prog() -> Program {
    Program {
        name: "Accounting".to_string(),
        semesters: vec![
            vec![CC!("ACCT", 1010), CC!("ACCT", 1030)],
            vec![CC!("ACCT", 1040), CC!("ACCT", 1050)],
        ],
        assoc_stems: vec!["ACCT".to_string()],
        electives: vec![],
    }
}
