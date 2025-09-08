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
        (CC!("PSYC", 2000), PreCourse(CC!("PSYC", 1000))),
        (CC!("PSYC", 2010), PreCourse(CC!("PSYC", 2000))),
        (CC!("PSYC", 2401), PreCourse(CC!("PSYC", 1000))),
        (CC!("PSYC", 2503), PreCourse(CC!("PSYC", 1000))),
        (CC!("PSYC", 2631), PreCourse(CC!("PSYC", 1000))),
        (CC!("PSYC", 2641), PreCourse(CC!("PSYC", 1000))),
        (CC!("PSYC", 2731), PreCourse(CC!("PSYC", 1000))),
        (
            CC!("PSYC", 2852),
            Or(vec![
                PreCourse(CC!("PSYC", 1000)),
                PreCourse(CC!("SOCI", 1000)),
            ]),
        ),
        (
            CC!("PSYC", 3152),
            Or(vec![
                And(vec![
                    PreCourse(CC!("PSYC", 2010)),
                    PreCourse(CC!("PSYC", 3901)),
                ]),
                Instructor,
            ]),
        ),
        (
            CC!("PSYC", 3500),
            And(vec![PreCourse(CC!("PSYC", 2010)), Instructor]),
        ),
        (CC!("PSYC", 3710), PreCourse(CC!("PSYC", 2010))),
        (CC!("PSYC", 3720), PreCourse(CC!("PSYC", 1000))),
        (CC!("PSYC", 3801), PreCourse(CC!("PSYC", 1000))),
        (
            CC!("PSYC", 3901),
            And(vec![
                PreCourse(CC!("PSYC", 1000)),
                Standing(ClassStanding::Sophomore),
                PreCourse(CC!("PSYC", 2731)),
            ]),
        ),
        (
            CC!("PSYC", 4012),
            And(vec![
                Or(vec![
                    PreCourse(CC!("PSYC", 2731)),
                    PreCourse(CC!("PSYC", 3901)),
                ]),
                Standing(ClassStanding::Junior),
                PreCourse(CC!("PSYC", "PROGRAM")),
            ]),
        ),
        (
            CC!("PSYC", 4050),
            And(vec![
                PreCourse(CC!("PSYC", 2010)),
                PreCourse(CC!("BIOL", 1107)),
            ]),
        ),
        (CC!("PSYC", 4210), Standing(ClassStanding::Junior)),
        (CC!("PSYC", 4502), Standing(ClassStanding::Junior)),
        (
            CC!("PSYC", 4820),
            And(vec![
                Or(vec![
                    PreCourse(CC!("PSYC", 1000)),
                    PreCourse(CC!("SOCI", 1000)),
                ]),
                Standing(ClassStanding::Junior),
            ]),
        ),
        (
            CC!("PSYC", 4850),
            And(vec![
                PreCourse(CC!("PSYC", "PROGRAM")),
                Standing(ClassStanding::Junior),
            ]),
        ),
        (
            CC!("PSYC", 4910),
            Or(vec![PreCourse(CC!("PSYC", "SE")), Instructor]),
        ),
        (CC!("PSYC", 4975), Instructor),
    ]
}
