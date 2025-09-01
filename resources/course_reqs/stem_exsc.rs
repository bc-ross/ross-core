#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("EXSC", 1112), PreCourse(CC!("EXSC", 1108))),
        (CC!("EXSC", 1128), Instructor),
        (
            CC!("EXSC", 3310),
            Or(vec![
                And(vec![
                    PreCourse(CC!("EXSC", 2240)),
                    PreCourse(CC!("EXSC", 2263)),
                    PreCourse(CC!("EXSC", 3303)),
                ]),
                Instructor,
            ]),
        ),
        (
            CC!("EXSC", 3366),
            Or(vec![
                PreCourse(CC!("BIOL", 2242)),
                PreCourse(CC!("BIOL", 2243)),
                PreCourse(CC!("EXSC", 2240)),
            ]),
        ),
        (
            CC!("EXSC", 3380),
            Or(vec![
                PreCourse(CC!("BIOL", 2242)),
                PreCourse(CC!("BIOL", 2243)),
                PreCourse(CC!("EXSC", 2240)),
            ]),
        ),
    ]
}
