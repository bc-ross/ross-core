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
            CC!("stem ARCH", 1200),
            Or(vec![
                PreCourse(CC!("ART", 1000)),
                CoCourse(CC!("ART", 1000)),
            ]),
        ),
        (
            CC!("stem ARCH", 1410),
            And(vec![
                Or(vec![
                    PreCourse(CC!("STEM ARCH", 1300)),
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
                PreCourse(CC!("STEM ARCH", 1200)),
                PreCourse(CC!("STEM ARCH", 1410)),
                PreCourse(CC!("MATH", 1300)),
                CoCourse(CC!("STEM ARCH", 2201)),
            ]),
        ),
        (
            CC!("stem ARCH", 2112),
            PreCourseGrade(CC!("STEM ARCH", 2111), GR!(C)),
        ),
        (
            CC!("stem ARCH", 2201),
            And(vec![
                PreCourse(CC!("ART", 1000)),
                PreCourse(CC!("STEM ARCH", 1200)),
                PreCourse(CC!("STEM ARCH", 1410)),
            ]),
        ),
        (CC!("stem ARCH", 3100), PreCourse(CC!("STEM ARCH", 2112))),
        (
            CC!("stem ARCH", 3113),
            And(vec![
                PreCourse(CC!("STEM ARCH", 2112)),
                PreCourse(CC!("STEM ARCH", 2301)),
                Or(vec![
                    PreCourse(CC!("PHYS", 2000)),
                    PreCourse(CC!("PHYS", 2100)),
                ]),
                Or(vec![
                    PreCourse(CC!("STEM ARCH", 2300)),
                    CoCourse(CC!("STEM ARCH", 2300)),
                ]),
            ]),
        ),
        (
            CC!("stem ARCH", 3114),
            PreCourseGrade(CC!("STEM ARCH", 3113), GR!(C)),
        ),
        (CC!("stem ARCH", 3200), PreCourse(CC!("STEM ARCH", 2201))),
        (CC!("stem ARCH", 3310), PreCourse(CC!("STEM ARCH", 2112))),
        (
            CC!("stem ARCH", 3400),
            And(vec![
                PreCourse(CC!("STEM ARCH", 2301)),
                PreCourse(CC!("STEM ARCH", 2112)),
            ]),
        ),
        (
            CC!("stem ARCH", 4116),
            And(vec![
                PreCourse(CC!("STEM ARCH", 4115)),
                PreCourse(CC!("STEM ARCH", 4400)),
            ]),
        ),
        (CC!("stem ARCH", 4400), PreCourse(CC!("STEM ARCH", 3113))),
    ]
}
