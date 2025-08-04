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
            CC!("ACCT", 3270),
            Or(vec![PreCourseGrade(CC!("ACCT", 2090), GR!(C)), Instructor]),
        ),
        (
            CC!("ACCT", 3640),
            Or(vec![PreCourse(CC!("FINC", 3100)), Instructor]),
        ),
    ]
}
