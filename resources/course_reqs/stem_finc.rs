#![allow(unused_imports)]

use crate::prereqs::{
    CourseReq::{self, *},
    Grade, GradeLetter, GradeQualifier,
};
use crate::schedule::CourseCode;
use crate::{CC, GR};

pub fn prereqs() -> Vec<(CourseCode, CourseReq)> {
    vec![
        (CC!("FINC", 3100), PreCourse(CC!("ACCT", 2090))),
        (
            CC!("FINC", 3300),
            And(vec![
                PreCourse(CC!("FINC", 3100)),
                PreCourse(CC!("ECON", 2900)),
            ]),
        ),
        (CC!("FINC", 3950), PreCourse(CC!("MGMT", 2250))),
        (CC!("FINC", 4100), PreCourse(CC!("FINC", 3100))),
        (
            CC!("FINC", 4300),
            And(vec![
                PreCourse(CC!("FINC", 3300)),
                PreCourse(CC!("FINC", 3060)),
            ]),
        ),
        (
            CC!("FINC", 4330),
            And(vec![
                PreCourse(CC!("FINC", 3300)),
                PreCourse(CC!("FINC", 3060)),
            ]),
        ),
        (CC!("FINC", 4650), PreCourse(CC!("FINC", 3100))),
        (CC!("FINC", 4790), Instructor),
        (CC!("FINC", 4900), PreCourse(CC!("FINC", 3100))),
        (CC!("FINC", 4910), PreCourse(CC!("FINC", 3100))),
        (CC!("FINC", 4950), PreCourse(CC!("FINC", 4100))),
        (CC!("FINC", 6590), PreCourse(CC!("ACCT", 5510))),
    ]
}
