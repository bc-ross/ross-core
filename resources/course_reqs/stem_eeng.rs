#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("EENG", 1050), PreCourse(CC!("EENG", 1000))),
        (CC!("EENG", 2070), CoCourse(CC!("EENG", 2060))),
    ]
}
