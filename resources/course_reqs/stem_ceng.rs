#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("CENG", 2010), PreCourse(CC!("CHEM", 1210))),
        (CC!("CENG", 3050), PreCourse(CC!("CENG", 2010))),
        (
            CC!("CENG", 3250),
            PreCourseGrade(CC!("CENG", 2010), GR!(C+)),
        ),
        (
            CC!("CENG", 3300),
            PreCourseGrade(CC!("CENG", 2010), GR!(C+)),
        ),
        (
            CC!("CENG", 3350),
            PreCourseGrade(CC!("CENG", 2010), GR!(C+)),
        ),
        (
            CC!("CENG", 4080),
            And(vec![
                And(vec![
                    PreCourse(CC!("CENG", 3050)),
                    PreCourse(CC!("CENG", 4210)),
                ]),
                CoCourse(CC!("CENG", 3050)),
                CoCourse(CC!("CENG", 4210)),
            ]),
        ),
        (
            CC!("CENG", 3050),
            And(vec![
                PreCourse(CC!("CENG", 2010)),
                CoCourse(CC!("ENGR", 3600)),
            ]),
        ),
        (
            CC!("CENG", 3350),
            And(vec![
                PreCourseGrade(CC!("CENG", 2010), GR!(C+)),
                CoCourse(CC!("ENGR", 3150)),
            ]),
        ),
        (
            CC!("CENG", 4210),
            And(vec![
                PreCourse(CC!("CENG", 2010)),
                PreCourse(CC!("MATH", 2100)),
                CoCourse(CC!("ENGR", 3600)),
            ]),
        ),
        (
            CC!("CENG", 4350),
            And(vec![
                PreCourse(CC!("CENG", 4210)),
                PreCourse(CC!("ENGR", 3410)),
            ]),
        ),
        (
            CC!("CENG", 4600),
            And(vec![
                PreCourse(CC!("CENG", 4080)),
                PreCourse(CC!("ENGR", 3170)),
            ]),
        ),
        (CC!("CENG", 4610), PreCourse(CC!("CENG", 4600))),
        (CC!("CENG", 4810), PreCourse(CC!("CENG", 4210))),
        (
            CC!("CENG", 4820),
            And(vec![
                And(vec![
                    PreCourse(CC!("CHEM", 3500)),
                    PreCourse(CC!("CENG", 4210)),
                ]),
                CoCourse(CC!("CHEM", 3500)),
                CoCourse(CC!("CENG", 4210)),
            ]),
        ),
        (
            CC!("CENG", 4830),
            And(vec![
                And(vec![
                    PreCourse(CC!("ENGR", 3600)),
                    PreCourse(CC!("CENG", 3050)),
                ]),
                CoCourse(CC!("ENGR", 3600)),
                CoCourse(CC!("CENG", 3050)),
            ]),
        ),
        (
            CC!("CENG", 4850),
            And(vec![
                Or(vec![
                    PreCourse(CC!("CSCI", 2300)),
                    PreCourse(CC!("ENGR", 2000)),
                ]),
                PreCourse(CC!("CENG", 3050)),
                PreCourse(CC!("CENG", 4210)),
            ]),
        ),
        (
            CC!("CENG", 4860),
            And(vec![
                PreCourse(CC!("CENG", 2010)),
                PreCourse(CC!("ENGR", 3250)),
                PreCourse(CC!("ENGR", 3500)),
            ]),
        ),
        (
            CC!("CENG", 4870),
            And(vec![
                Or(vec![
                    PreCourse(CC!("CENG", 2300)),
                    PreCourse(CC!("ENGR", 2000)),
                ]),
                PreCourse(CC!("CHEM", 3800)),
                PreCourse(CC!("ENGR", 3250)),
            ]),
        ),
    ]
}
