#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("CHEM", 1010), CoCourse(CC!("CHEM", 1011))),
        (CC!("CHEM", 1011), CoCourse(CC!("CHEM", 1010))),
        (CC!("CHEM", 1200), CoCourse(CC!("BIOL", 1121))), // (CC!("CHEM", 1200), CoCourse(CC!("CHEM", 1201))),
        (CC!("CHEM", 1201), CoCourse(CC!("CHEM", 1200))),
        (
            CC!("CHEM", 1210),
            And(vec![
                PreCourse(CC!("CHEM", 1201)),
                CoCourse(CC!("CHEM", 1211)),
            ]),
        ),
        (CC!("CHEM", 1211), CoCourse(CC!("CHEM", 1210))),
        (
            CC!("CHEM", 2200),
            And(vec![
                PreCourse(CC!("CHEM", 1210)),
                PreCourse(CC!("CHEM", 1211)),
                CoCourse(CC!("CHEM", 2201)),
            ]),
        ),
        (CC!("CHEM", 2201), CoCourse(CC!("CHEM", 2200))),
        (
            CC!("CHEM", 2210),
            And(vec![
                PreCourse(CC!("CHEM", 2200)),
                CoCourse(CC!("CHEM", 2211)),
            ]),
        ),
        (
            CC!("CHEM", 2211),
            And(vec![
                PreCourse(CC!("CHEM", 2201)),
                CoCourse(CC!("CHEM", 2210)),
            ]),
        ),
        (
            CC!("CHEM", 3150),
            Or(vec![
                Instructor,
                And(vec![
                    PreCourse(CC!("CHEM", 1210)),
                    PreCourse(CC!("MATH", 1350)),
                    PreCourse(CC!("PHYS", 2110)),
                ]),
            ]),
        ),
        (CC!("CHEM", 3250), PreCourse(CC!("CHEM", 2200))),
        (
            CC!("CHEM", 3300),
            And(vec![
                PreCourse(CC!("CHEM", 1210)),
                PreCourse(CC!("CHEM", 1211)),
                CoCourse(CC!("CHEM", 3301)),
            ]),
        ),
        (CC!("CHEM", 3301), CoCourse(CC!("CHEM", 3300))),
        (
            CC!("CHEM", 3311),
            And(vec![
                PreCourse(CC!("CHEM", 3300)),
                PreCourse(CC!("CHEM", 3301)),
            ]),
        ),
        (
            CC!("CHEM", 3400),
            And(vec![
                PreCourse(CC!("CHEM", 2210)),
                PreCourse(CC!("CHEM", 2211)),
                CoCourse(CC!("CHEM", 3401)),
            ]),
        ),
        (
            CC!("CHEM", 3500),
            And(vec![
                PreCourse(CC!("CHEM", 2210)),
                PreCourse(CC!("CHEM", 2211)),
                CoCourse(CC!("CHEM", 3501)),
            ]),
        ),
        (CC!("CHEM", 3501), CoCourse(CC!("CHEM", 3500))),
        (
            CC!("CHEM", 3510),
            And(vec![
                PreCourse(CC!("CHEM", 3500)),
                CoCourse(CC!("CHEM", 3511)),
            ]),
        ),
        (CC!("CHEM", 3511), CoCourse(CC!("CHEM", 3510))),
    ]
}
