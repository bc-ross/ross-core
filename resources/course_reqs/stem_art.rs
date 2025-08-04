use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PREREQS: Vec<(CourseCode, CourseReq)> = vec![
        (CC!("ART", 2110), PreCourse(CC!("ART", 1000)),),
        (
            CC!("ART", 2200),
            Or(vec![
                PreCourse(CC!("ART", 1000)),
                PreCourse(CC!("ART", 1030))
            ]),
        ),
    ];
}
