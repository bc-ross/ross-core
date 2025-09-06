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
        (
            CC!("MENG", 3180),
            And(vec![
                PreCourse(CC!("ENGR", 1500)),
                PreCourse(CC!("ENGR", 3500)),
                CoCourse(CC!("ENGR", 3500)),
            ]),
        ),
        (
            CC!("MENG", 3220),
            And(vec![
                PreCourse(CC!("ENGR", 2000)),
                PreCourse(CC!("ENGR", 2310)),
                PreCourse(CC!("ENGR", 2320)),
            ]),
        ),
        (
            CC!("MENG", 3240),
            And(vec![
                PreCourse(CC!("MENG", 3220)),
                PreCourse(CC!("MENG", 3180)),
                CoCourse(CC!("MENG", 3180)),
            ]),
        ),
        (
            CC!("MENG", 3820),
            Or(vec![
                And(vec![
                    PreCourse(CC!("ENGR", 1500)),
                    PreCourse(CC!("ENGR", 3150)),
                ]),
                And(vec![
                    CoCourse(CC!("ENGR", 1500)),
                    CoCourse(CC!("ENGR", 3150)),
                ]),
            ]),
        ),
        (
            CC!("MENG", 4240),
            Or(vec![
                And(vec![
                    PreCourse(CC!("ENGR", 2000)),
                    PreCourse(CC!("ENGR", 2310)),
                    PreCourse(CC!("MATH", 3100)),
                ]),
                And(vec![
                    CoCourse(CC!("ENGR", 2000)),
                    CoCourse(CC!("ENGR", 2310)),
                    CoCourse(CC!("ENGR", 3100)),
                ]),
            ]),
        ),
        (
            CC!("MENG", 4600),
            And(vec![
                PreCourse(CC!("MENG", 3240)),
                PreCourse(CC!("ENGR", 3170)),
            ]),
        ),
        (CC!("MENG", 4610), PreCourse(CC!("MENG", 4600))),
        (CC!("MENG", 4700), PreCourse(CC!("MENG", 3240))),
        (CC!("MENG", 4730), PreCourse(CC!("MENG", 4240))),
        (CC!("MENG", 4810), PreCourse(CC!("MENG", 4240))),
        (CC!("MENG", 4820), PreCourse(CC!("ENGR", 2320))),
        (
            CC!("MENG", 4830),
            Or(vec![PreCourse(CC!("MENG", 3240)), Instructor]),
        ),
        (
            CC!("MENG", 4840),
            And(vec![
                PreCourse(CC!("ENGR", 2000)),
                PreCourse(CC!("MATH", 1350)),
            ]),
        ),
        (
            CC!("MENG", 4850),
            And(vec![
                PreCourse(CC!("MENG", 3250)),
                PreCourse(CC!("MENG", 3600)),
            ]),
        ),
        (CC!("MENG", 4860), PreCourse(CC!("MENG", 3250))),
        (
            CC!("MENG", 4870),
            And(vec![
                And(vec![
                    PreCourse(CC!("ENGR", 2000)),
                    PreCourse(CC!("MENG", 3220)),
                ]),
                And(vec![
                    PreCourse(CC!("EENG", 2060)),
                    PreCourse(CC!("EENG", 3060)),
                ]),
            ]),
        ),
        (
            CC!("MENG", 4910),
            And(vec![
                PreCourse(CC!("ENGR", 1500)),
                PreCourse(CC!("ENGR", 3250)),
                PreCourse(CC!("ENGR", 3300)),
            ]),
        ),
        (
            CC!("MENG", 4920),
            And(vec![
                PreCourse(CC!("MATH", 3100)),
                PreCourse(CC!("ENGR", 2000)),
            ]),
        ),
    ]
}
