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
        (CC!("ENGR", 1001), PreCourse(CC!("MATH", 1020))),
        (
            CC!("ENGR", 1200),
            Or(vec![
                Or(vec![
                    PreCourse(CC!("MATH", 1250)),
                    PreCourse(CC!("MATH", 1300)),
                ]),
                Or(vec![
                    CoCourse(CC!("MATH", 1250)),
                    CoCourse(CC!("MATH", 1300)),
                ]),
            ]),
        ),
        (CC!("ENGR", 1520), PreCourse(CC!("ENGR", 1500))),
        (CC!("ENGR", 2000), CoCourse(CC!("ENGR", 1200))),
        (
            CC!("ENGR", 2300),
            Or(vec![
                PreCourse(CC!("PHYS", 2100)),
                PreCourse(CC!("PHYS", 2000)),
            ]),
        ),
        (
            CC!("ENGR", 2310),
            Or(vec![
                And(vec![
                    PreCourse(CC!("ENGR", 2300)),
                    PreCourse(CC!("ENGR", 2000)),
                ]),
                PreCourse(CC!("CIVL", 2000)),
            ]),
        ),
        (CC!("ENGR", 2320), PreCourse(CC!("ENGR", 2300))),
        (
            CC!("ENGR", 3150),
            And(vec![
                PreCourse(CC!("MATH", 2300)),
                CoCourse(CC!("MATH", 2300)),
            ]),
        ),
        (
            CC!("ENGR", 3250),
            And(vec![
                PreCourse(CC!("PHYS", 2100)),
                PreCourse(CC!("MATH", 1350)),
            ]),
        ),
        (
            CC!("ENGR", 3300),
            And(vec![
                PreCourse(CC!("MATH", 2300)),
                PreCourse(CC!("PHYS", 2100)),
                Or(vec![
                    PreCourse(CC!("ENGR", 2300)),
                    PreCourse(CC!("CENG", 2010)),
                ]),
            ]),
        ),
        (
            CC!("ENGR", 3400),
            Or(vec![
                PreCourse(CC!("ENGR", 2320)),
                PreCourse(CC!("ENGR", 3500)),
            ]),
        ),
        (
            CC!("ENGR", 3410),
            And(vec![
                PreCourse(CC!("ENGR", 3150)),
                PreCourse(CC!("ENGR", 3600)),
                CoCourse(CC!("ENGR", 3600)),
            ]),
        ),
        (CC!("ENGR", 3500), PreCourse(CC!("CHEM", 1200))),
        (
            CC!("ENGR", 3600),
            Or(vec![
                PreCourse(CC!("ENGR", 3300)),
                And(vec![
                    PreCourse(CC!("CENG", 3300)),
                    PreCourse(CC!("ENGR", 3250)),
                ]),
                PreCourse(CC!("CENG", 3250)),
            ]),
        ),
        (CC!("ENGR", 4150), PreCourse(CC!("ENGR", 3150))),
        (CC!("ENGR", 4840), PreCourse(CC!("ENGR", 3150))),
    ]
}
