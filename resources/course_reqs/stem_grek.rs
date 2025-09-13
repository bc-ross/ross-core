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
            CC!("GREK", 1020),
            Or(vec![
                PreCourse(CC!("GREK", 1000)),
                PreCourse(CC!("GREK", "E1000")),
            ]),
        ),
        (
            CC!("GREK", 2120),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CC!("GREK", "E1020")),
            ]),
        ),
        (
            CC!("GREK", 3110),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CC!("GREK", "E1020")),
            ]),
        ),
        (
            CC!("GREK", 3120),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CC!("GREK", "E1020")),
            ]),
        ),
        (
            CC!("GREK", 4110),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CC!("GREK", "E1020")),
            ]),
        ),
        (
            CC!("GREK", 4120),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CC!("GREK", "E1020")),
            ]),
        ),
    ]
}
