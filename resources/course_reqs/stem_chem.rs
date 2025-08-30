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
            CC!("CHEM", 2200),
            And(vec![
                PreCourse(CC!("CHEM", 1210)),
                PreCourse(CC!("CHEM", 1211)),
            ]),
        ),
        (CC!("CHEM", 1210), PreCourse(CC!("CHEM", 1201))),
        (CC!("CHEM", 2210), PreCourse(CC!("CHEM", 2200))),
        (CC!("CHEM", 2211), PreCourse(CC!("CHEM", 2201))),
        (
            CC!("CHEM", 3150),
            And(vec![
                PreCourse(CC!("CHEM", 1210)),
                PreCourse(CC!("MATH", 1350)),
                PreCourse(CC!("PHYS", 2110)),
            ]),
        ),
        (CC!("CHEM", 3250), PreCourse(CC!("CHEM", 2200))),
        (
            CC!("CHEM", 3300),
            And(vec![
                PreCourse(CC!("CHEM", 1210)),
                PreCourse(CC!("CHEM", 1211)),
            ]),
        ),
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
            ]),
        ),
        (
            CC!("CHEM", 3500),
            And(vec![
                PreCourse(CC!("CHEM", 2210)),
                PreCourse(CC!("CHEM", 2211)),
            ]),
        ),
        (CC!("CHEM", 3510), PreCourse(CC!("CHEM", 2500))),
        (CC!("CHEM", 3510), PreCourse(CC!("CHEM", 3500))),
        (CC!("CHEM", terminate), PreCourse(CC!("CHEM", "EXIT"))),
    ]
}
