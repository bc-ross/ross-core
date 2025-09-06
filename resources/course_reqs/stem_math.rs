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
        (CC!("MATH", 1040), Instructor),
        (CC!("MATH", 1110), PreCourse(CC!("MATH", "PROGRAM"))),
        (
            CC!("MATH", 1300),
            Or(vec![
                PreCourseGrade(CC!("MATH", 1250), GR!(C)),
                PreCourse(CC!("MATH", "E1250")),
            ]),
        ),
        (CC!("MATH", 1120), PreCourse(CC!("MATH", 1110))),
        (CC!("MATH", 1130), PreCourse(CC!("MATH", 1110))),
        (CC!("MATH", 1350), PreCourse(CC!("MATH", 1300))),
        (CC!("MATH", 2300), PreCourse(CC!("MATH", 1350))),
        (CC!("MATH", 2500), PreCourse(CC!("MATH", 1300))),
        (CC!("MATH", 2900), PreCourse(CC!("MATH", 1300))),
        (CC!("MATH", 3100), PreCourse(CC!("MATH", 2300))),
        (CC!("MATH", 3200), PreCourse(CC!("MATH", 2300))),
        (
            CC!("MATH", 3300),
            And(vec![
                PreCourse(CC!("MATH", 1350)),
                Or(vec![
                    PreCourse(CC!("CSCI", 1140)),
                    PreCourse(CC!("CSCI", 2300)),
                    PreCourse(CC!("ENGR", 2000)),
                ]),
            ]),
        ),
        (
            CC!("MATH", 3400),
            Or(vec![PreCourse(CC!("MATH", 2250)), Instructor]),
        ),
        (
            CC!("MATH", 3600),
            And(vec![
                PreCourse(CC!("MATH", 2500)),
                PreCourse(CC!("MATH", 2550)),
            ]),
        ),
        (
            CC!("MATH", 3610),
            And(vec![
                PreCourse(CC!("MATH", 2500)),
                PreCourse(CC!("MATH", 2550)),
            ]),
        ),
        (
            CC!("MATH", 4457),
            And(vec![
                PreCourse(CC!("MATH", "PROGRAM")),
                PreCourse(CC!("EDUC", 3332)),
            ]),
        ),
        (
            CC!("MATH", 4600),
            And(vec![
                PreCourse(CC!("MATH", 2500)),
                PreCourse(CC!("MATH", 2550)),
            ]),
        ),
        (CC!("MATH", 4700), PreCourse(CC!("MATH", 2300))),
        (
            CC!("MATH", 4800),
            And(vec![
                PreCourse(CC!("MATH", 2300)),
                PreCourse(CC!("MATH", 2550)),
            ]),
        ),
        (
            CC!("MATH", 4930),
            And(vec![
                Or(vec![
                    Standing(ClassStanding::Junior),
                    PreCourse(CC!("MATH", "SE")),
                ]),
                PreCourse(CC!("MATH", "PROGRAM")),
                Instructor,
            ]),
        ),
    ]
}
