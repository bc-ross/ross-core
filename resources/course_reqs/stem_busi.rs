#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("BUSI", 3902), PreCourse(CC!("BUSI", 3902))),
        (CC!("BUSI", 3903), PreCourse(CC!("BUSI", 3902))),
        (CC!("BUSI", 3904), PreCourse(CC!("BUSI", 3903))),
        (CC!("BUSI", 3905), PreCourse(CC!("BUSI", 3904))),
        (CC!("BUSI", 3906), PreCourse(CC!("BUSI", 3905))),
        (CC!("BUSI", 4250), PreCourse(CC!("MGMT", 3250))),
        (CC!("BUSI", 4450), PreCourse(CC!("MGMT", 3250))),
        (
            CC!("BUSI", 4860),
            And(vec![
                PreCourse(CC!("THEO", 2000)),
                PreCourse(CC!("PHIL", 3250)),
            ]),
        ),
    ]
}
