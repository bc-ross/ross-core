#![allow(unused_imports)]

use crate::prereqs::{
    ClassStanding,
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("MCOM", 1610), Instructor),
        (CC!("MCOM", 3320), PreCourse(CC!("MCOM", 2000))),
        (CC!("MCOM", 3325), PreCourse(CC!("MCOM", 3225))),
        (
            CC!("MCOM", 3330),
            Or(vec![PreCourse(CC!("JOUR", 2620)), Instructor]),
        ),
        (CC!("MCOM", 3610), PreCourse(CC!("MCOM", 2610))),
        (CC!("MCOM", 4090), PreCourse(CC!("MCOM", "SE"))),
        (CC!("MCOM", 4220), PreCourse(CC!("MCOM", 3225))),
        (
            CC!("MCOM", 4330),
            And(vec![
                PreCourse(CC!("JOUR", 2620)),
                PreCourse(CC!("MCOM", 3330)),
            ]),
        ),
        (CC!("MCOM", 4680), PreCourse(CC!("MCOM", 3680))),
    ]
}
