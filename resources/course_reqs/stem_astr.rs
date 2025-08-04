#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("ASTR", 1050), PreCourse(CC!("ASTR", 1010))),
        (
            CC!("ASTR", 1080),
            Or(vec![
                PreCourse(CC!("ASTR", 1010)),
                PreCourse(CC!("ASTR", 1050)),
            ]),
        ),
    ]
}
