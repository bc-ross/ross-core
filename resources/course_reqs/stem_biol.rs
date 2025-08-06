#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("BIOL", 2260), PreCourse(CC!("BIOL", 1121))),
        (
            CC!("BIOL", 3305),
            And(vec![
                PreCourse(CC!("BIOL", 1121)),
                PreCourse(CC!("BIOL", 1122)),
            ]),
        ),
        (
            CC!("BIOL", 3310),
            And(vec![
                PreCourse(CC!("BIOL", 1121)),
                PreCourse(CC!("BIOL", 1122)),
            ]),
        ),
        (CC!("BIOL", 3312), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 3313), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 3345), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 3346), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 3346), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 3347), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 3353), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 3354), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 3355), PreCourse(CC!("CHEM", 1210))),
        (
            CC!("BIOL", 3360),
            Or(vec![
                PreCourse(CC!("CHEM", 2200)),
                PreCourse(CC!("BIOL", 3370)),
            ]),
        ),
        (CC!("BIOL", 3370), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 4410), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 4475), PreCourse(CC!("CHEM", 2200))),
        (CC!("BIOL", 4476), PreCourse(CC!("CHEM", 2200))),
        (CC!("BIOL", 4482), PreCourse(CC!("CHEM", 1210))),
        (CC!("BIOL", 4484), PreCourse(CC!("CHEM", 2200))),
        (CC!("BIOL", 4486), PreCourse(CC!("CHEM", 1210))),
    ]
}
