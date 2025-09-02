#![allow(unused_imports)]

use crate::prereqs::{
    ClassStanding,
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![(
        CC!("GRBK", 3750),
        And(vec![
            PreCourse(CC!("GRBK", 1750)),
            PreCourse(CC!("GRBK", 2750)),
            PreCourse(CC!("GRBK", 2850)),
        ]),
    )]
}
