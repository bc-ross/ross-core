use crate::{
    CC,
    geneds::{
        GenEd::{self, *},
        GenEdReq::{self, *},
    },
    schedule::CourseCode,
};

pub fn geneds() -> Vec<GenEd> {
    vec![
        Core {
            name: "English Composition".to_string(),
            req: Courses {
                num: 1,
                courses: vec![CC!("ENGL", 1000), CC!("ENGL", 1010), CC!("HONR", 1030)],
            },
        },
        Core {
            name: "Intro to Theo".to_string(),
            req: Courses {
                num: 1,
                courses: vec![CC!("THEO", 1100)],
            },
        },
        Core {
            name: "Wellness for Life".to_string(),
            req: Courses {
                num: 1,
                courses: vec![CC!("EXSC", 1115), CC!("NURS", 3200)],
            },
        },
        Core {
            name: "Natural Philosophy".to_string(),
            req: Courses {
                num: 1,
                courses: vec![CC!("PHIL", 2100), CC!("PHIL", 2310)],
            },
        },
        Core {
            name: "Foreign Language".to_string(),
            req: SetOpts(vec![
                vec![CC!("SPAN", 1000), CC!("SPAN", 1020)],
                vec![CC!("GREK", 1000), CC!("GREK", 1020)],
                vec![CC!("ITAL", 1000), CC!("ITAL", 1020)],
                vec![CC!("LATN", 1000), CC!("LATN", 1020)],
                vec![CC!("FREN", 1000), CC!("FREN", 1020)],
                vec![CC!("THEO", 2010), CC!("THEO", 2020)],
                vec![CC!("ESLG", 2930)],
            ]),
        },
        Core {
            name: "Physical Fitness".to_string(),
            req: Courses {
                num: 1,
                courses: vec![
                    CC!("EXSC", 1100),
                    CC!("EXSC", 1101),
                    CC!("EXSC", 1105),
                    CC!("EXSC", 1106),
                    CC!("EXSC", 1107),
                    CC!("EXSC", 1108),
                    CC!("EXSC", 1109),
                    CC!("EXSC", 1111),
                    CC!("EXSC", 1114),
                    CC!("EXSC", 1116),
                    CC!("EXSC", 1117),
                    CC!("EXSC", 1126),
                    CC!("EXSC", 1128),
                    CC!("EXSC", 1129),
                    CC!("MILS", 1160),
                    CC!("MILS", 2160),
                    CC!("MILS", 3160),
                ],
            },
        },
    ]
}
