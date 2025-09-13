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
            CC!("LATN", 1020),
            Or(vec![
                PreCourse(CC!("LATN", 1000)),
                PreCourse(CourseCode {
                    stem: "LATN".into(),
                    code: CourseCodeSuffix::Unique(1000, "EXAM".into()),
                }),
            ]),
        ),
        (
            CC!("LATN", 3110),
            Or(vec![
                PreCourse(CC!("LATN", 1020)),
                PreCourse(CourseCode {
                    stem: "LATN".into(),
                    code: CourseCodeSuffix::Unique(1020, "EXAM".into()),
                }),
            ]),
        ),
        (
            CC!("LATN", 3120),
            Or(vec![
                PreCourse(CC!("LATN", 1020)),
                PreCourse(CourseCode {
                    stem: "LATN".into(),
                    code: CourseCodeSuffix::Unique(1020, "EXAM".into()),
                }),
            ]),
        ),
        (
            CC!("LATN", 4110),
            Or(vec![
                PreCourse(CC!("LATN", 1020)),
                PreCourse(CourseCode {
                    stem: "LATN".into(),
                    code: CourseCodeSuffix::Unique(1020, "EXAM".into()),
                }),
            ]),
        ),
        (
            CC!("LATN", 4120),
            Or(vec![
                PreCourse(CC!("LATN", 1020)),
                PreCourse(CourseCode {
                    stem: "LATN".into(),
                    code: CourseCodeSuffix::Unique(1020, "EXAM".into()),
                }),
            ]),
        ),
    ]
}
