#![allow(unused_imports)]

use crate::schedule::CourseCode;
use crate::{CC, GR};
use crate::{
    prereqs::{
        ClassStanding,
        CourseReq::{self, *},
        Grade, GradeLetter, GradeQualifier,
    },
    schedule::CourseCodeSuffix,
};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (
            CC!("GREK", 1020),
            Or(vec![
                PreCourse(CC!("GREK", 1000)),
                PreCourse(CourseCode {
                    stem: "GREK".into(),
                    code: CourseCodeSuffix::Unique(1000, "EXAM".into()),
                }),
            ]),
        ),
        (
            CC!("GREK", 2120),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CourseCode {
                    stem: "GREK".into(),
                    code: CourseCodeSuffix::Unique(1020, "EXAM".into()),
                }),
            ]),
        ),
        (
            CC!("GREK", 3110),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CourseCode {
                    stem: "GREK".into(),
                    code: CourseCodeSuffix::Unique(1020, "EXAM".into()),
                }),
            ]),
        ),
        (
            CC!("GREK", 3120),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CourseCode {
                    stem: "GREK".into(),
                    code: CourseCodeSuffix::Unique(1020, "EXAM".into()),
                }),
            ]),
        ),
        (
            CC!("GREK", 4110),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CourseCode {
                    stem: "GREK".into(),
                    code: CourseCodeSuffix::Unique(1020, "EXAM".into()),
                }),
            ]),
        ),
        (
            CC!("GREK", 4120),
            Or(vec![
                PreCourse(CC!("GREK", 1020)),
                PreCourse(CourseCode {
                    stem: "GREK".into(),
                    code: CourseCodeSuffix::Unique(1020, "EXAM".into()),
                }),
            ]),
        ),
    ]
}
