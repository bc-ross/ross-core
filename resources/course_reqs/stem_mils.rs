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
        (CC!("MILS", 1260), PreCourse(CC!("MILS", 1160))),
        (
            CC!("MILS", 1480),
            And(vec![
                PreCourse(CC!("MILS", 1440)),
                CoCourse(CC!("MILS", 1000)),
            ]),
        ),
        (CC!("MILS", 2160), PreCourse(CC!("MILS", 1260))),
        (CC!("MILS", 3020), CoCourse(CC!("MILS", 3160))),
        (CC!("MILS", 3120), CoCourse(CC!("MILS", 3260))),
        (CC!("MILS", 3160), CoCourse(CC!("MILS", 3020))),
        (
            CC!("MILS", 3260),
            And(vec![
                PreCourse(CC!("MILS", 3160)),
                CoCourse(CC!("MILS", 3120)),
            ]),
        ),
        (CC!("MILS", 4020), CoCourse(CC!("MILS", 4160))),
        (
            CC!("MILS", 4040),
            And(vec![
                PreCourse(CC!("MILS", 3480)),
                CoCourse(CC!("MILS", 1000)),
            ]),
        ),
        (
            CC!("MILS", 4080),
            And(vec![
                PreCourse(CC!("MILS", 4040)),
                CoCourse(CC!("MILS", 1000)),
            ]),
        ),
        (CC!("MILS", 4120), CoCourse(CC!("MILS", 4260))),
        (CC!("MILS", 4160), CoCourse(CC!("MILS", 4020))),
        (
            CC!("MILS", 4260),
            And(vec![
                PreCourse(CC!("MILS", 4160)),
                CoCourse(CC!("MILS", 4120)),
            ]),
        ),
    ]
}
