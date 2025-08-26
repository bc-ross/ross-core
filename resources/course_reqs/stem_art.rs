#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("ART", 2110), PreCourse(CC!("ART", 1000))),
        (
            CC!("ART", 2200),
            Or(vec![
                PreCourse(CC!("ART", 1000)),
                PreCourse(CC!("ART", 1030)),
            ]),
        ),
        (CC!("ART", 2300), PreCourse(CC!("ART", 1010))),
        (CC!("ART", 2500), PreCourse(CC!("ART", 1000))),
        (CC!("ART", 3001), PreCourse(CC!("ART", 1000))),
        (CC!("ART", 3002), PreCourse(CC!("ART", 3001))),
        (CC!("ART", 3111), PreCourse(CC!("ART", 2110))),
        (CC!("ART", 3112), PreCourse(CC!("ART", 3111))),
        (CC!("ART", 3121), PreCourse(CC!("ART", 1000))),
        (CC!("ART", 3122), PreCourse(CC!("ART", 3131))),
        (CC!("ART", 3131), Instructor),
        (CC!("ART", 3132), Instructor),
        (CC!("ART", 3201), PreCourse(CC!("ART", 2200))),
        (CC!("ART", 3202), PreCourse(CC!("ART", 3201))),
        (
            CC!("ART", 3210),
            Or(vec![
                PreCourse(CC!("ART", 1000)),
                PreCourse(CC!("ART", 1030)),
            ]),
        ),
        (CC!("ART", 3301), PreCourse(CC!("ART", 2300))),
        (CC!("ART", 3302), PreCourse(CC!("ART", 3301))),
        (CC!("ART", 3310), PreCourse(CC!("ART", 2300))),
        (CC!("ART", 3501), PreCourse(CC!("ART", 2500))),
        (CC!("ART", 3502), PreCourse(CC!("ART", 3501))),
        (CC!("ART", 3801), PreCourse(CC!("ART", 2800))),
        (CC!("ART", 3802), PreCourse(CC!("ART", 3801))),
        (CC!("ART", 4000), PreCourse(CC!("ART", 3002))),
        (CC!("ART", 4110), PreCourse(CC!("ART", 3112))),
        (CC!("ART", 4200), PreCourse(CC!("ART", 3202))),
        (CC!("ART", 4300), PreCourse(CC!("ART", 3302))),
        (CC!("ART", 4461), PreCourse(CC!("ART", 4460))),
        (CC!("ART", 4462), PreCourse(CC!("ART", 4461))),
        (CC!("ART", 4500), PreCourse(CC!("ART", 1000))),
        (CC!("ART", 4800), PreCourse(CC!("ART", 3802))),
        (CC!("ART", 4900), PreCourse(CC!("ART", 3900))),
        (CC!("ART", 4901), PreCourse(CC!("ART", 4900))),
        (
            CC!("ART", 4950),
            And(vec![
                PreCourse(CC!("ART", 4310)),
                PreCourse(CC!("ART", 4311)),
            ]),
        ),
    ]
}
