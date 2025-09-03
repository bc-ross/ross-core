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
        (CC!("MGMT", 2550), PreCourse(CC!("BUSI", 1550))),
        (CC!("MGMT", 3250), PreCourse(CC!("MGMT", 2250))),
        (CC!("MGMT", 3400), PreCourse(CC!("MGMT", 2250))),
        (
            CC!("MGMT", 3510),
            And(vec![
                PreCourse(CC!("MGMT", 2250)),
                PreCourse(CC!("ACCT", 2090)),
            ]),
        ),
        (
            CC!("MGMT", 3550),
            Or(vec![Standing(ClassStanding::Junior), Instructor]),
        ),
        (
            CC!("MGMT", 3660),
            And(vec![
                PreCourse(CC!("MGMT", 2250)),
                Or(vec![
                    PreCourse(CC!("BUSI", 2650)),
                    PreCourse(CC!("MATH", 1220)),
                ]),
            ]),
        ),
        (CC!("MGMT", 4450), PreCourse(CC!("MGMT", 3450))),
        (
            CC!("MGMT", 4500),
            And(vec![
                PreCourse(CC!("MGMT", 2250)),
                PreCourse(CC!("MGMT", 3500)),
            ]),
        ),
        (CC!("MGMT", 4560), PreCourse(CC!("MGMT", 2250))),
        (
            CC!("MGMT", 4660),
            And(vec![
                PreCourse(CC!("MGMT", 2250)),
                Or(vec![
                    PreCourse(CC!("BUSI", 2650)),
                    PreCourse(CC!("MATH", 1220)),
                ]),
            ]),
        ),
        (CC!("MGMT", 4720), PreCourse(CC!("BUSI", 3710))),
        (CC!("MGMT", 4730), PreCourse(CC!("BUSI", 4500))),
        (CC!("MGMT", 4740), PreCourse(CC!("BUSI", 3710))),
        (CC!("MGMT", 4750), PreCourse(CC!("MGMT", 2250))),
        (
            CC!("MGMT", 4780),
            And(vec![Standing(ClassStanding::Junior), Instructor]),
        ),
        (CC!("MGMT", 4890), Instructor),
    ]
}
