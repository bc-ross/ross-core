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
            CC!("ATHC", 2212),
            And(vec![
                PreCourse(CC!("EXSC", 2210)),
                PreCourse(CC!("EXSC", 2263)),
            ]),
        ),
        (
            CC!("ATHC", 2213),
            And(vec![
                PreCourse(CC!("EXSC", 2210)),
                PreCourse(CC!("EXSC", 2263)),
            ]),
        ),
        (
            CC!("ATHC", 3312),
            Or(vec![
                And(vec![
                    PreCourse(CC!("EXSC", 2209)),
                    PreCourse(CC!("ATHC", 3364)),
                    PreCourse(CC!("ATHC", 3374)),
                ]),
                And(vec![
                    PreCourse(CC!("BIOL", 2242)),
                    PreCourse(CC!("BIOL", 2243)),
                ]),
            ]),
        ),
        (CC!("ATHC", 3313), Instructor),
        (
            CC!("ATHC", 3361),
            And(vec![
                PreCourse(CC!("EXSC", 2210)),
                PreCourse(CC!("EXSC", 2263)),
                PreCourse(CC!("EXSC", 3380)),
            ]),
        ),
        (
            CC!("ATHC", 3362),
            And(vec![
                PreCourse(CC!("EXSC", 2210)),
                PreCourse(CC!("EXSC", 2263)),
            ]),
        ),
        (
            CC!("ATHC", 3364),
            Or(vec![
                PreCourse(CC!("EXSC", 2263)),
                And(vec![
                    PreCourse(CC!("BIOL", 2242)),
                    PreCourse(CC!("BIOL", 2243)),
                ]),
            ]),
        ),
        (
            CC!("ATHC", 3374),
            And(vec![
                PreCourse(CC!("EXSC", 2263)),
                PreCourse(CC!("BIOL", 2242)),
                PreCourse(CC!("BIOL", 2243)),
            ]),
        ),
        (CC!("ATHC", 4406), PreCourse(CC!("EXSC", 2263))),
        (
            CC!("ATHC", 4407),
            Or(vec![
                PreCourse(CC!("EXSC", 2263)),
                PreCourse(CC!("EXSC", 3366)),
            ]),
        ),
        (
            CC!("ATHC", 4412),
            And(vec![
                PreCourse(CC!("ATHC", 3364)),
                PreCourse(CC!("ATHC", 4406)),
            ]),
        ),
        (CC!("ATHC", 4413), PreCourse(CC!("ATHC", 4412))),
    ]
}
