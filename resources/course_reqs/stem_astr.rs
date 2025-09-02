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
        (CC!("ASTR", 3000), PreCourse(CC!("PHYS", 2110))),
        (CC!("ASTR", 4100), PreCourse(CC!("PHYS", 3200))),
        (CC!("ASTR", 4200), PreCourse(CC!("PHYS", 2110))),
        (
            CC!("ASTR", 4300),
            And(vec![
                Standing(ClassStanding::Junior),
                PreCourse(CC!("PHYS", 3200)),
            ]),
        ),
    ]
}
