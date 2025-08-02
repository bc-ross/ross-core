use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PREREQS_MAP: HashMap<CourseCode, CourseReq> = HashMap::from([(
        CC!("CS", 1050),
        Or(vec![
            And(vec![
                PreCourseGrade(CC!("CS", 1080), GR!(C-)),
                PreCourse(CC!("CS", 1090)),
                PreCourseGrade(CC!("MATH", 1090), GR!(B)),
                PreCourse(CC!("CS", 1040))
            ]),
            CoCourseGrade(CC!("CS", 1050), GR!(C+)),
            CoCourseGrade(CC!("MUSC", 1090), GR!(F)),
            PreCourse(CC!("CS", 1030)),
            PreCourse(CC!("CS", COMP)),
            PreCourse(CC!("CS", COMP)),
            PreCourse(CC!("MATH", "COMP")),
            PreCourseGrade(CC!("MATH", "COMP"), GR!(C+))
        ]),
    ),]);
}
