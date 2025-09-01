#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("EVCA", 3100), PreCourse(CC!("THEO", 1100))),
        (CC!("EVCA", 3150), PreCourse(CC!("EVCA", 3100))),
        (CC!("EVCA", 4700), Instructor),
    ]
}
