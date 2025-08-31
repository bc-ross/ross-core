#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (
            CC!("DANC", 2020),
            And(vec![
                PreCourse(CC!("DANC", 1010)),
                PreCourse(CC!("DANC", 2055)),
            ]),
        ),
        (CC!("DANC", 2040), PreCourse(CC!("DANC", 1010))),
        (CC!("DANC", 2050), PreCourse(CC!("DANC", 1010))),
        (CC!("DANC", 2055), PreCourse(CC!("DANC", 1010))),
        (CC!("DANC", 2065), PreCourse(CC!("DANC", 1010))),
        (CC!("DANC", 2080), PreCourse(CC!("DANC", 1010))),
        (CC!("DANC", 2085), PreCourse(CC!("DANC", 1010))),
        (
            CC!("DANC", 3010),
            And(vec![
                PreCourse(CC!("DANC", 2040)),
                PreCourse(CC!("DANC", 2055)),
                PreCourse(CC!("DANC", 2080)),
            ]),
        ),
        (CC!("DANC", 3500), PreCourse(CC!("DANC", 3010))),
    ]
}
