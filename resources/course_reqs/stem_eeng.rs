#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("EENG", 2020), CoCourse(CC!("EENG", 2010))),
        (CC!("EENG", 2060), PreCourse(CC!("PHYS", 2110))),
        (CC!("EENG", 3060), CoCourse(CC!("EENG", 2060))),
        (CC!("EENG", 3070), CoCourse(CC!("EENG", 3130))),
        (CC!("EENG", 3080), PreCourse(CC!("EENG", 3070))),
        (
            CC!("EENG", 3130),
            And(vec![
                PreCourse(CC!("EENG", 2060)),
                CoCourse(CC!("MATH", 3100)),
            ]),
        ),
        (
            CC!("EENG", 3140),
            And(vec![
                PreCourse(CC!("EENG", 3130)),
                CoCourse(CC!("MATH", 3100)),
            ]),
        ),
        (
            CC!("EENG", 3160),
            And(vec![
                PreCourse(CC!("EENG", 2060)),
                CoCourse(CC!("MATH", 3100)),
            ]),
        ),
        (
            CC!("EENG", 3210),
            And(vec![
                PreCourse(CC!("EENG", 3130)),
                CoCourse(CC!("EENG", 3080)),
            ]),
        ),
        (CC!("EENG", 4010), PreCourse(CC!("EENG", 3210))),
        (
            CC!("EENG", 4020),
            And(vec![
                PreCourse(CC!("EENG", 3210)),
                CoCourse(CC!("EENG", 4010)),
            ]),
        ),
        (
            CC!("EENG", 4050),
            And(vec![
                PreCourse(CC!("EENG", 3140)),
                PreCourse(CC!("MATH", 3100)),
            ]),
        ),
        (CC!("EENG", 4060), CoCourse(CC!("EENG", 4050))),
        (
            CC!("EENG", 4090),
            And(vec![
                PreCourse(CC!("EENG", 3130)),
                PreCourse(CC!("EENG", 3160)),
            ]),
        ),
        (
            CC!("EENG", 4210),
            And(vec![
                PreCourse(CC!("EENG", 3210)),
                CoCourse(CC!("EENG", 3090)),
            ]),
        ),
        (CC!("EENG", 4220), PreCourse(CC!("EENG", 3080))),
        (
            CC!("EENG", 4510),
            And(vec![
                PreCourse(CC!("EENG", 2010)),
                PreCourse(CC!("CSCI", 2300)),
            ]),
        ),
        (
            CC!("EENG", 4520),
            And(vec![
                PreCourse(CC!("EENG", 2010)),
                PreCourse(CC!("CSCI", 2300)),
                PreCourse(CC!("EENG", 3210)),
                CoCourse(CC!("EENG", 4530)),
            ]),
        ),
        (CC!("EENG", 4530), CoCourse(CC!("EENG", 4520))),
        (
            CC!("EENG", 4600),
            And(vec![
                PreCourse(CC!("EENG", 4050)),
                PreCourse(CC!("EENG", 4210)),
                PreCourse(CC!("EENG", 4520)),
            ]),
        ),
        (CC!("EENG", 4610), PreCourse(CC!("EENG", 4600))),
        (
            CC!("EENG", 4810),
            And(vec![
                PreCourse(CC!("EENG", 3210)),
                PreCourse(CC!("EENG", 4090)),
            ]),
        ),
        (CC!("EENG", 4820), PreCourse(CC!("EENG", 3130))),
        (CC!("EENG", 4910), PreCourse(CC!("EENG", 4210))),
        (CC!("EENG", 4940), PreCourse(CC!("EENG", 3140))),
    ]
}
