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
            CC!("ENGL", 3010),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3030),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3040),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3050),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3060),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3070),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3110),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3120),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3130),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3140),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 3150),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (CC!("ENGL", 3200), Instructor),
        (CC!("ENGL", 3201), Instructor),
        (
            CC!("ENGL", 3260),
            Or(vec![
                PreCourse(CC!("ENGL", 1010)),
                PreCourse(CC!("ENGL", 1030)),
            ]),
        ),
        (
            CC!("ENGL", 3357),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 4010),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 4020),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 4040),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 4050),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 4060),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 4110),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 4130),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 4140),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (
            CC!("ENGL", 4200),
            Or(vec![
                PreCourse(CC!("ENGL", 1500)),
                PreCourse(CC!("ENGL", 1550)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
        (CC!("ENGL", 4250), PreCourse(CC!("ENGL", 3250))),
        (
            CC!("ENGL", 4500),
            Or(vec![
                PreCourse(CC!("THEO", 1100)),
                PreCourse(CC!("ENGL", 1600)),
                PreCourse(CC!("ENGL", 1650)),
                PreCourse(CC!("ENGL", 1700)),
                PreCourse(CC!("ENGL", 1750)),
            ]),
        ),
    ]
}
