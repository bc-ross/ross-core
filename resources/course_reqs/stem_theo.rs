#![allow(unused_imports)]

use crate::prereqs::{
    ClassStanding,
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("THEO", 2000), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 2100), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 2144), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 2150), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3100), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3133), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3144), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3150), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3160), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3200), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3220), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3230), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3240), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3260), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3420), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3430), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3620), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3640), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3660), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3680), PreCourse(CC!("PHIL", 3670))),
        (
            CC!("THEO", 3690),
            And(vec![
                PreCourse(CC!("PHIL", 3670)),
                PreCourse(CC!("THEO", 3680)),
                CoCourse(CC!("PHIL", 3690)),
            ]),
        ),
        (CC!("THEO", 3820), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3840), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3920), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3940), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3950), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3960), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 3970), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 4000), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 4457), PreCourse(CC!("THEO", 1100))),
        (CC!("THEO", 4500), PreCourse(CC!("THEO", "PROGRAM"))),
        (CC!("THEO", 4980), PreCourse(CC!("THEO", 1100))),
    ]
}
