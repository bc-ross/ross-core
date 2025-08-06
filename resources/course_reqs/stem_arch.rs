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
            CC!("ARCH", 1200),
            Or(vec![
                PreCourse(CC!("ART", 1000)),
                CoCourse(CC!("ART", 1000)),
            ]),
        ),
        (
            CC!("ARCH", 1410),
            And(vec![
                Or(vec![
                    PreCourse(CC!("ARCH", 1300)),
                    PreCourse(CC!("ART", 1000)),
                ]),
                Or(vec![
                    PreCourse(CC!("ART", 1010)),
                    PreCourse(CC!("ART", 1030)),
                ]),
                And(vec![
                    CoCourse(CC!("ART", 1000)),
                    Or(vec![CoCourse(CC!("ART", 1010)), CoCourse(CC!("ART", 1030))]),
                ]),
            ]),
        ),
        (
            CC!("stem ARCH", 2111),
            And(vec![
                PreCourse(CC!("ARCH", 1200)),
                PreCourse(CC!("ARCH", 1410)),
                PreCourse(CC!("MATH", 1300)),
                CoCourse(CC!("ARCH", 2201)),
            ]),
        ),
        (
            CC!("stem ARCH", 2112),
            PreCourseGrade(CC!("ARCH", 2111), GR!(C)),
        ),
        (
            CC!("stem ARCH", 2201),
            And(vec![
                PreCourse(CC!("ART", 1000)),
                PreCourse(CC!("ARCH", 1200)),
                PreCourse(CC!("ARCH", 1410)),
            ]),
        ),
        (CC!("ARCH", 3100), PreCourse(CC!("ARCH", 2112))),
        (
            CC!("ARCH", 3113),
            And(vec![
                PreCourse(CC!("ARCH", 2112)),
                PreCourse(CC!("ARCH", 2301)),
                Or(vec![
                    PreCourse(CC!("PHYS", 2000)),
                    PreCourse(CC!("PHYS", 2100)),
                ]),
                Or(vec![
                    PreCourse(CC!("ARCH", 2300)),
                    CoCourse(CC!("ARCH", 2300)),
                ]),
            ]),
        ),
        (
            CC!("ARCH", 3114),
            PreCourseGrade(CC!("ARCH", 3113), GR!(C)),
        ),
        (CC!("ARCH", 3200), PreCourse(CC!("ARCH", 2201))),
        (CC!("ARCH", 3310), PreCourse(CC!("ARCH", 2112))),
        (
            CC!("ARCH", 3400),
            And(vec![
                PreCourse(CC!("ARCH", 2301)),
                PreCourse(CC!("ARCH", 2112)),
            ]),
        ),
        (
            CC!("ARCH", 4116),
            And(vec![
                PreCourse(CC!("ARCH", 4115)),
                PreCourse(CC!("ARCH", 4400)),
            ]),
        ),
        (CC!("ARCH", 4400), PreCourse(CC!("ARCH", 3113))),
    ]
}
