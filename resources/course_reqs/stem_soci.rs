#![allow(unused_imports)]

use crate::prereqs::{
    ClassStanding,
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (
            CC!("SOCI", 3155),
            Or(vec![
                PreCourse(CC!("SOCI", 1000)),
                PreCourse(CC!("CRIM", 1000)),
            ]),
        ),
        (CC!("SOCI", 4175), PreCourse(CC!("SOCI", 3155))),
        (CC!("SOCI", 4176), PreCourse(CC!("SOCI", 4175))),
    ]
}
