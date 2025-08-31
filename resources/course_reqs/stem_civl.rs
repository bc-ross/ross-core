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
            CC!("CIVL", 3010),
            And(vec![
                PreCourse(CC!("ENGR", 2320)),
                PreCourse(CC!("ENGR", 3150)),
                PreCourse(CC!("ENGL", 1010)),
            ]),
        ),
        (
            CC!("CIVL", 3020),
            And(vec![
                PreCourse(CC!("ENGR", 3150)),
                PreCourse(CC!("CIVL", 3310)),
                PreCourse(CC!("ENGL", 1010)),
            ]),
        ),
        (CC!("CIVL", 3120), PreCourse(CC!("ENGR", 2320))),
        (CC!("CIVL", 3230), PreCourse(CC!("ENGR", 3300))),
        (
            CC!("CIVL", 3310),
            And(vec![
                PreCourse(CC!("ENGR", 3300)),
                Or(vec![
                    PreCourse(CC!("CIVL", 2310)),
                    PreCourse(CC!("CHEM", 1210)),
                ]),
            ]),
        ),
        (CC!("CIVL", 3510), PreCourse(CC!("ENGR", 2320))),
        (CC!("CIVL", 3550), PreCourse(CC!("ENGR", 2320))),
        (CC!("CIVL", 4140), PreCourse(CC!("CIVL", 3120))),
        (CC!("CIVL", 4160), PreCourse(CC!("CIVL", 3120))),
        (CC!("CIVL", 4210), PreCourse(CC!("ENGR", 3300))),
        (CC!("CIVL", 4320), PreCourse(CC!("CIVL", 3310))),
        (CC!("CIVL", 4510), PreCourse(CC!("CIVL", 3510))),
        (CC!("CIVL", 4530), PreCourse(CC!("CIVL", 4530))),
        (CC!("CIVL", 4530), PreCourse(CC!("CIVL", 3510))),
    ]
}
