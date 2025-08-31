#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (
            CC!("ECON", 3000),
            And(vec![
                PreCourse(CC!("ECON", 2090)),
                PreCourse(CC!("ECON", 2100)),
            ]),
        ),
        (
            CC!("ECON", 3060),
            And(vec![
                PreCourse(CC!("ECON", 2090)),
                PreCourse(CC!("ECON", 2100)),
            ]),
        ),
        (
            CC!("ECON", 3120),
            And(vec![
                PreCourse(CC!("ECON", 3090)),
                PreCourse(CC!("ECON", 3100)),
            ]),
        ),
        (
            CC!("ECON", 3150),
            And(vec![
                PreCourse(CC!("ECON", 2090)),
                PreCourse(CC!("ECON", 2100)),
            ]),
        ),
        (
            CC!("ECON", 3200),
            And(vec![
                PreCourse(CC!("ECON", 2090)),
                PreCourse(CC!("ECON", 2100)),
            ]),
        ),
        (
            CC!("ECON", 4110),
            And(vec![
                PreCourse(CC!("ECON", 3090)),
                PreCourse(CC!("ECON", 3100)),
            ]),
        ),
        (CC!("ECON", 4160), PreCourse(CC!("ECON", 3100))),
    ]
}
