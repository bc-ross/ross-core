#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("CSCI", 2000), Instructor),
        (CC!("CSCI", 2150), PreCourse(CC!("CSCI", 1140))),
        (CC!("CSCI", 2300), PreCourse(CC!("MATH", 1300))),
        (
            CC!("CSCI", 2650),
            And(vec![
                PreCourse(CC!("MATH", 2550)),
                PreCourse(CC!("CSCI", 1140)),
            ]),
        ),
        (
            CC!("CSCI", 3100),
            And(vec![
                PreCourse(CC!("MATH", 2550)),
                PreCourse(CC!("CSCI", 2150)),
            ]),
        ),
        (
            CC!("CSCI", 3500),
            And(vec![
                PreCourse(CC!("CSCI", 2150)),
                PreCourse(CC!("CSCI", 2560)),
            ]),
        ),
        (
            CC!("CSCI", 3570),
            And(vec![
                PreCourse(CC!("CSCI", 2150)),
                PreCourse(CC!("CSCI", 2560)),
            ]),
        ),
        (CC!("CSCI", 3600), PreCourse(CC!("CSCI", 2150))),
        (CC!("CSCI", 3800), PreCourse(CC!("CSCI", 2150))),
        (
            CC!("CSCI", 4200),
            And(vec![
                PreCourse(CC!("MATH", 2550)),
                PreCourse(CC!("CSCI", 2150)),
            ]),
        ),
        (
            CC!("CSCI", 4400),
            And(vec![
                PreCourse(CC!("MATH", 2550)),
                PreCourse(CC!("CSCI", 2150)),
            ]),
        ),
        (CC!("CSCI", 4790), Instructor),
        (CC!("CSCI", 4900), CoCourse(CC!("CSCI", "COMP"))),
        (CC!("CSCI", 4930), PreCourse(CC!("CSCI", 4920))),
    ]
}
