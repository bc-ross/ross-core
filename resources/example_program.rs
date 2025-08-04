
use crate::CC;
use crate::schedule::{CourseCode, Program};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROG: Program = Program {
        name: "Example #2".to_string(),
        semesters: vec![
            vec![CC!("ASTR", 1050), CC!("ENGL", 1060), CC!("HONR", "PROJ")],
            vec![CC!("ENGL", 9930)],
        ],
        electives: vec![],
        assoc_stems: vec!["ENGL".to_string(), "ASTR".to_string(), "COLO".to_string()],
    };
}
