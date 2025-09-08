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
            CC!("SPAN", 2010),
            Or(vec![
                PreCourse(CC!("SPAN", 1020)),
                CoCourse(CC!("SPAN", "E1020")),
            ]),
        ),
        (
            CC!("SPAN", 2020),
            Or(vec![
                PreCourse(CC!("SPAN", 2010)),
                CoCourse(CC!("SPAN", "E2010")),
            ]),
        ),
        (
            CC!("SPAN", 3010),
            Or(vec![
                PreCourse(CC!("SPAN", 2020)),
                CoCourse(CC!("SPAN", "E2020")),
            ]),
        ),
        (
            CC!("SPAN", 3020),
            Or(vec![
                PreCourse(CC!("SPAN", 2020)),
                CoCourse(CC!("SPAN", "E2020")),
            ]),
        ),
        (
            CC!("SPAN", 3040),
            Or(vec![
                PreCourse(CC!("SPAN", 2020)),
                CoCourse(CC!("SPAN", "E2020")),
            ]),
        ),
        (
            CC!("SPAN", 3400),
            Or(vec![
                PreCourse(CC!("SPAN", 2020)),
                CoCourse(CC!("SPAN", "E2020")),
            ]),
        ),
        (CC!("SPAN", 3650), PreCourse(CC!("SPAN", 3040))),
        (CC!("SPAN", 3660), PreCourse(CC!("SPAN", 3040))),
        (
            CC!("SPAN", 3710),
            Or(vec![
                PreCourse(CC!("SPAN", 2020)),
                CoCourse(CC!("SPAN", "E2020")),
            ]),
        ),
        (
            CC!("SPAN", 3720),
            Or(vec![
                PreCourse(CC!("SPAN", 2020)),
                CoCourse(CC!("SPAN", "E2020")),
            ]),
        ),
        (
            CC!("SPAN", 3750),
            Or(vec![
                PreCourse(CC!("SPAN", 2020)),
                CoCourse(CC!("SPAN", "E2020")),
            ]),
        ),
        (
            CC!("SPAN", 3800),
            Or(vec![
                PreCourse(CC!("SPAN", 2010)),
                CoCourse(CC!("SPAN", "E2010")),
            ]),
        ),
        (
            CC!("SPAN", 3801),
            Or(vec![
                PreCourse(CC!("SPAN", 2010)),
                CoCourse(CC!("SPAN", "E2010")),
            ]),
        ),
        (
            CC!("SPAN", 3802),
            Or(vec![
                PreCourse(CC!("SPAN", 2010)),
                CoCourse(CC!("SPAN", "E2010")),
            ]),
        ),
        (
            CC!("SPAN", 3803),
            Or(vec![
                PreCourse(CC!("SPAN", 2010)),
                CoCourse(CC!("SPAN", "E2010")),
            ]),
        ),
        (
            CC!("SPAN", 3804),
            Or(vec![
                PreCourse(CC!("SPAN", 2010)),
                CoCourse(CC!("SPAN", "E2010")),
            ]),
        ),
        (
            CC!("SPAN", 3805),
            Or(vec![
                PreCourse(CC!("SPAN", 2010)),
                CoCourse(CC!("SPAN", "E2010")),
            ]),
        ),
        (CC!("SPAN", 4700), PreCourse(CC!("SPAN", 3040))),
        (CC!("SPAN", 4710), PreCourse(CC!("SPAN", "PROGRAM"))),
        (CC!("SPAN", 4720), PreCourse(CC!("SPAN", "PROGRAM"))),
        (CC!("SPAN", 4800), PreCourse(CC!("SPAN", 3040))),
        (CC!("SPAN", 4810), PreCourse(CC!("SPAN", 3400))),
    ]
}
