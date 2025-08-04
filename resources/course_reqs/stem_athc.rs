use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PREREQS: Vec<(CourseCode, CourseReq)> = vec![(
        CC!("ATHC", 2212),
        And(vec![
            PreCourse(CC!("EXSC", 2210)),
            PreCourse(CC!("EXSC", 2263))
        ]),
    ),];
}
