#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("FREN", 1020), PreCourse(CC!("FREN", 1000))),
        (
            CC!("FREN", 2010),
            Or(vec![
                PreCourse(CC!("FREN", 1020)),
                PreCourse(CC!("FREN", "E1020")),
            ]),
        ),
        (
            CC!("FREN", 3010),
            Or(vec![
                PreCourse(CC!("FREN", 1020)),
                PreCourse(CC!("FREN", "E1020")),
            ]),
        ),
        (CC!("FREN", 3040), PreCourse(CC!("FREN", 2010))),
        (CC!("FREN", 3300), PreCourse(CC!("FREN", 2010))),
        (
            CC!("FREN", 3510),
            Or(vec![
                PreCourse(CC!("FREN", 2010)),
                PreCourse(CC!("FREN", "E2010")),
            ]),
        ),
        (
            CC!("FREN", 3610),
            Or(vec![
                PreCourse(CC!("FREN", 2010)),
                PreCourse(CC!("FREN", "E2010")),
            ]),
        ),
        (
            CC!("FREN", 3620),
            Or(vec![
                PreCourse(CC!("FREN", 2010)),
                PreCourse(CC!("FREN", "E2010")),
            ]),
        ),
        (
            CC!("FREN", 3630),
            Or(vec![
                PreCourse(CC!("FREN", 2010)),
                PreCourse(CC!("FREN", "E2010")),
            ]),
        ),
        (
            CC!("FREN", 3640),
            Or(vec![
                PreCourse(CC!("FREN", 2010)),
                PreCourse(CC!("FREN", "E2010")),
            ]),
        ),
        (
            CC!("FREN", 3650),
            Or(vec![
                PreCourse(CC!("FREN", 2010)),
                PreCourse(CC!("FREN", "E2010")),
            ]),
        ),
        (CC!("FREN", 4710), Instructor),
        (CC!("FREN", 4720), Instructor),
    ]
}
