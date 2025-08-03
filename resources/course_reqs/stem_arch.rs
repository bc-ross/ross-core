use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PREREQS: Vec<(CourseCode, CourseReq)> = vec![
        (CC!("ARCH", 1200), PreCourse(CC!("ART", 1000)),),
        (CC!("ARCH", 3100), PreCourse(CC!("ARCH", 2101)),),
    ];
}
