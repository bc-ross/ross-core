use crate::prereqs::CourseReq::{self, *};
use crate::schedule::CourseCode;
use crate::CC;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PREREQS_MAP: HashMap<CourseCode, CourseReq> = HashMap::from([(
        CC!("CS", 1050),
        And(vec![
            PreCourse(CC!("CS", 1040)),
            PreCourse(CC!("BUSI", 1090))
        ]),
    ),]);
}
