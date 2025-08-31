#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("CRIM", 3000), PreCourse(CC!("CRIM", 1000))),
        (CC!("CRIM", 3050), PreCourse(CC!("CRIM", 1000))),
        (CC!("CRIM", 3220), PreCourse(CC!("CRIM", 3100))),
    ]
}
