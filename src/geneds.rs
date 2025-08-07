use std::collections::{HashMap, HashSet};

use crate::schedule::{CourseCode, Schedule};
use anyhow::Result;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub enum GenEd {
    Core { name: String, req: GenEdReq },
    Foundation { name: String, req: GenEdReq },
    SkillAndPerspective { name: String, req: GenEdReq },
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub enum GenEdReq {
    Set(Vec<CourseCode>),
    SetOpts(Vec<Vec<CourseCode>>),
    Courses {
        num: usize,
        courses: Vec<CourseCode>,
    },
    Credits {
        num: usize,
        courses: Vec<CourseCode>,
    },
}

impl GenEd {
    pub fn fulfilled_courses(
        &self,
        all_codes: &HashSet<&CourseCode>,
    ) -> Option<HashSet<&CourseCode>> {
        match self {
            GenEd::Core { req, .. } => req.fulfilled_courses(all_codes),
            GenEd::Foundation { req, .. } => req.fulfilled_courses(all_codes),
            GenEd::SkillAndPerspective { req, .. } => req.fulfilled_courses(all_codes),
        }
    }
}

impl GenEdReq {
    fn fulfilled_courses(&self, all_codes: &HashSet<&CourseCode>) -> Option<HashSet<&CourseCode>> {
        match self {
            GenEdReq::Set(codes) => {
                let fulfilled: HashSet<_> = codes
                    .iter()
                    .filter(|code| all_codes.contains(*code))
                    .collect();
                if fulfilled.len() == codes.len() {
                    Some(fulfilled)
                } else {
                    None
                }
            }
            GenEdReq::SetOpts(opts) => {
                for opt in opts {
                    let fulfilled: HashSet<_> = opt
                        .iter()
                        .filter(|code| all_codes.contains(*code))
                        .collect();
                    if fulfilled.len() == opt.len() {
                        return Some(fulfilled);
                    }
                }
                None
            }
            GenEdReq::Courses { num, courses } => {
                let fulfilled: HashSet<_> = courses
                    .iter()
                    .filter(|code| all_codes.contains(*code))
                    .collect();
                if fulfilled.len() >= *num {
                    Some(fulfilled)
                } else {
                    None
                }
            }
            GenEdReq::Credits { num, courses } => {
                let fulfilled: HashSet<_> = courses
                    .iter()
                    .filter(|code| all_codes.contains(*code))
                    .collect();
                if fulfilled.len() >= *num {
                    Some(fulfilled)
                } else {
                    None
                }
            }
        }
    }
}

pub fn are_geneds_satisfied(sched: &Schedule) -> Result<bool> {
    let all_codes = sched
        .courses
        .iter()
        .flatten()
        .collect::<HashSet<&CourseCode>>();
    let mut foundation_courses = HashSet::new();
    let mut skill_and_perspective_courses = HashMap::new();
    for gened in &sched.catalog.geneds {
        match gened.fulfilled_courses(&all_codes) {
            Some(fulfilled) => match &gened {
                GenEd::Foundation { .. } => {
                    foundation_courses.extend(fulfilled);
                }
                GenEd::SkillAndPerspective { .. } => {
                    fulfilled.into_iter().for_each(|code| {
                        skill_and_perspective_courses
                            .entry(code)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    });
                }
                _ => {}
            },
            None => return Ok(false),
        }
    }
    Ok(true)
}
