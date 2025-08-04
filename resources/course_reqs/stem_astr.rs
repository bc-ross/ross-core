use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PREREQS: Vec<(CourseCode, CourseReq)> = vec![
        (CC!("ASTR", 3000), PreCourse(CC!("PHYS", 2110)),),
        (CC!("ASTR", 4100), PreCourse(CC!("PHYS", 3200)),),
    ];
}
