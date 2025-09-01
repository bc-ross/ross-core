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
            CC!("ESLG", 2220),
            Or(vec![PreCourse(CC!("ESLG", 2050)), Instructor]),
        ),
        (
            CC!("ESLG", 2930),
            Or(vec![PreCourse(CC!("ESLG", 2040)), Instructor]),
        ),
    ]
}
