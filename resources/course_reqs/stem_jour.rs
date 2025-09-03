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
        (CC!("JOUR", 3225), PreCourse(CC!("JOUR", 2620))),
        (CC!("JOUR", 3300), PreCourse(CC!("JOUR", 2620))),
        (CC!("JOUR", 3370), PreCourse(CC!("JOUR", 2620))),
        (CC!("JOUR", 4220), PreCourse(CC!("JOUR", 3225))),
        (
            CC!("JOUR", 4300),
            And(vec![
                PreCourse(CC!("JOUR", 2620)),
                PreCourse(CC!("JOUR", 3300)),
            ]),
        ),
        (CC!("JOUR", 4340), PreCourse(CC!("JOUR", 2620))),
    ]
}
