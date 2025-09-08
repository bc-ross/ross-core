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
            CC!("THTR", 2150),
            And(vec![
                PreCourse(CC!("THTR", 1150)),
                PreCourse(CC!("THTR", 1800)),
            ]),
        ),
        (
            CC!("THTR", 2250),
            And(vec![
                PreCourse(CC!("THTR", 1150)),
                PreCourse(CC!("THTR", 1800)),
            ]),
        ),
        (
            CC!("THTR", 3020),
            And(vec![
                PreCourse(CC!("THTR", 1150)),
                PreCourse(CC!("ENGL", 1010)),
            ]),
        ),
        (CC!("THTR", 3150), PreCourse(CC!("THTR", 2150))),
        (CC!("THTR", 3250), PreCourse(CC!("THTR", 2250))),
        (
            CC!("THTR", 3520),
            And(vec![
                PreCourse(CC!("THTR", 1550)),
                PreCourse(CC!("THTR", 1800)),
            ]),
        ),
        (
            CC!("THTR", 3540),
            And(vec![
                PreCourse(CC!("THTR", 1550)),
                PreCourse(CC!("THTR", 1800)),
            ]),
        ),
        (
            CC!("THTR", 3560),
            And(vec![
                PreCourse(CC!("THTR", 1550)),
                PreCourse(CC!("THTR", 1800)),
            ]),
        ),
        (
            CC!("THTR", 3580),
            And(vec![
                PreCourse(CC!("THTR", 1550)),
                PreCourse(CC!("THTR", 1800)),
            ]),
        ),
        (CC!("THTR", 3600), PreCourse(CC!("THTR", 1550))),
        (
            CC!("THTR", 3800),
            And(vec![
                PreCourse(CC!("THTR", 1550)),
                PreCourse(CC!("THTR", 1800)),
                PreCourse(CC!("ENGL", 1010)),
            ]),
        ),
        (
            CC!("THTR", 3810),
            And(vec![
                PreCourse(CC!("THTR", 1800)),
                PreCourse(CC!("ENGL", 1010)),
            ]),
        ),
        (
            CC!("THTR", 3820),
            And(vec![
                PreCourse(CC!("THTR", 1800)),
                PreCourse(CC!("ENGL", 1010)),
            ]),
        ),
        (
            CC!("THTR", 3830),
            And(vec![
                PreCourse(CC!("THTR", 1800)),
                PreCourse(CC!("ENGL", 1010)),
            ]),
        ),
        (CC!("THTR", 4150), PreCourse(CC!("THTR", 2245))),
        (CC!("THTR", 4790), Instructor),
    ]
}
