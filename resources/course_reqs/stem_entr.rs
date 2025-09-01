#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("ENTR", 3100), PreCourse(CC!("MGMT", 2250))),
        (CC!("ENTR", 4100), PreCourse(CC!("ENTR", 2100))),
        (
            CC!("ENTR", 4900),
            And(vec![
                PreCourse(CC!("ENTR", 3100)),
                PreCourse(CC!("ENTR", 3110)),
                PreCourse(CC!("ENTR", 3120)),
                PreCourse(CC!("ENTR", 4110)),
            ]),
        ),
    ]
}
