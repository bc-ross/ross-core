use crate::CC;
use crate::prereqs::CourseReq::{self, *};
use crate::schedule::CourseCode;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PREREQS_MAP: HashMap<CourseCode, CourseReq> = HashMap::from([
        (
            CC!("CS", 201),
            And(vec![PreCourse(CC!("CS", 102)), PreCourse(CC!("MATH", 101)),]),
        ),
        (
            CC!("CS", 202),
            Or(vec![PreCourse(CC!("CS", 201)), PreCourse(CC!("CS", 202)),]),
        ),
    ]);
}
