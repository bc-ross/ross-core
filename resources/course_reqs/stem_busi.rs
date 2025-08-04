use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PREREQS: Vec<(CourseCode, CourseReq)> =
        vec![(CC!("BUSI", 3902), PreCourse(CC!("BUSI", 3901)),),];
}
