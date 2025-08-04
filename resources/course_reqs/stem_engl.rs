#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("engl", 1030), PreCourse(CC!("engl", 1010))),
        (
            CC!("engl", 1090),
            Or(vec![
                PreCourse(CC!("engl", 1050)),
                PreCourse(CC!("engl", 1070)),
            ]),
        ),
    ]
}
