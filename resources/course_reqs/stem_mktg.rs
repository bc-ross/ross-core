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
        (CC!("MKTG", 3650), PreCourse(CC!("MKTG", 3100))),
        (CC!("MKTG", 3750), PreCourse(CC!("MKTG", 3100))),
        (CC!("MKTG", 3810), PreCourse(CC!("MKTG", 3100))),
        (CC!("MKTG", 3880), PreCourse(CC!("MKTG", 3100))),
        (CC!("MKTG", 4460), PreCourse(CC!("MKTG", 3100))),
        (CC!("MKTG", 4470), PreCourse(CC!("MKTG", 3100))),
        (CC!("MKTG", 4480), PreCourse(CC!("MKTG", 3100))),
        (
            CC!("MKTG", 4650),
            And(vec![
                PreCourse(CC!("MKTG", 3100)),
                PreCourse(CC!("MKTG", 3650)),
            ]),
        ),
        (
            CC!("MKTG", 4750),
            And(vec![
                PreCourse(CC!("MKTG", 3100)),
                PreCourse(CC!("MKTG", 3750)),
            ]),
        ),
        (
            CC!("MKTG", 4780),
            And(vec![
                Or(vec![
                    Standing(ClassStanding::Junior),
                    PreCourse(CC!("MKTG", "SE")),
                ]),
                Instructor,
            ]),
        ),
        (
            CC!("MKTG", 4790),
            And(vec![
                Or(vec![
                    Standing(ClassStanding::Junior),
                    PreCourse(CC!("MKTG", "SE")),
                ]),
                Instructor,
            ]),
        ),
        (CC!("MKTG", 4810), PreCourse(CC!("MKTG", 3100))),
        (
            CC!("MKTG", 4830),
            And(vec![
                PreCourse(CC!("MKTG", 3100)),
                PreCourse(CC!("BUSI", 2650)),
            ]),
        ),
        (
            CC!("MKTG", 4850),
            And(vec![
                PreCourse(CC!("MKTG", 3100)),
                PreCourse(CC!("MKTG", "SE")),
            ]),
        ),
    ]
}
