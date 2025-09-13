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
            CC!("LATN", 1020),
            Or(vec![
                PreCourse(CC!("LATN", 1000)),
                PreCourse(CC!("LATN", "E1000")),
            ]),
        ),
        (
            CC!("LATN", 3110),
            Or(vec![
                PreCourse(CC!("LATN", 1020)),
                PreCourse(CC!("LATN", "E1020")),
            ]),
        ),
        (
            CC!("LATN", 3120),
            Or(vec![
                PreCourse(CC!("LATN", 1020)),
                PreCourse(CC!("LATN", "E1020")),
            ]),
        ),
        (
            CC!("LATN", 4110),
            Or(vec![
                PreCourse(CC!("LATN", 1020)),
                PreCourse(CC!("LATN", "E1020")),
            ]),
        ),
        (
            CC!("LATN", 4120),
            Or(vec![
                PreCourse(CC!("LATN", 1020)),
                PreCourse(CC!("LATN", "E1020")),
            ]),
        ),
    ]
}
