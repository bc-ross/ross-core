use std::collections::{HashMap, HashSet};

use crate::schedule::{Catalog, CourseCode, Schedule};
use anyhow::Result;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};

const MAX_SKILLS_AND_PERSPECTIVES: u8 = 3;

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
        num: u32,
        courses: Vec<CourseCode>,
    },
}

impl GenEdReq {
    fn fulfilled_courses(
        &self,
        all_codes: &HashSet<&CourseCode>,
        catalog: &Catalog,
    ) -> Option<HashSet<&CourseCode>> {
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
                if fulfilled
                    .iter()
                    .filter_map(|c| catalog.courses.get(c).and_then(|(_, creds, _)| *creds))
                    .sum::<u32>()
                    >= *num
                {
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
    let mut skill_and_perspective_courses: HashMap<&CourseCode, u8> = HashMap::new();
    for gened in &sched.catalog.geneds {
        match gened {
            GenEd::Core { req, .. } => {
                if req.fulfilled_courses(&all_codes, &sched.catalog).is_none() {
                    return Ok(false);
                }
            }
            GenEd::Foundation { req, .. } => {
                match req.fulfilled_courses(
                    &all_codes
                        .difference(&foundation_courses)
                        .map(|x| *x)
                        .collect(),
                    &sched.catalog,
                ) {
                    Some(fulfilled) => {
                        foundation_courses.extend(fulfilled);
                    }
                    None => return Ok(false),
                }
            }
            GenEd::SkillAndPerspective { req, .. } => {
                match req.fulfilled_courses(
                    &all_codes
                        .difference(
                            &skill_and_perspective_courses
                                .iter()
                                .filter_map(|(x, count)| {
                                    if *count > MAX_SKILLS_AND_PERSPECTIVES {
                                        None
                                    } else {
                                        Some(*x)
                                    }
                                })
                                .collect(),
                        )
                        .map(|x| *x)
                        .collect(),
                    &sched.catalog,
                ) {
                    Some(fulfilled) => {
                        fulfilled.into_iter().for_each(|code| {
                            skill_and_perspective_courses
                                .entry(code)
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                        });
                    }
                    None => return Ok(false),
                }
            }
        }
    }
    Ok(true)
}

// Used for script_assistant crate
#[allow(dead_code)]
impl GenEd {
    pub fn all_course_codes(&self) -> Vec<CourseCode> {
        match self {
            GenEd::Core { req, .. } => req.all_course_codes(),
            GenEd::Foundation { req, .. } => req.all_course_codes(),
            GenEd::SkillAndPerspective { req, .. } => req.all_course_codes(),
        }
    }
}

impl GenEdReq {
    fn all_course_codes(&self) -> Vec<CourseCode> {
        let mut codes = Vec::new();
        self.collect_course_codes(&mut codes);
        codes.into_iter().map(|x| x.clone()).collect()
    }

    fn collect_course_codes<'a>(&'a self, codes: &mut Vec<&'a CourseCode>) {
        match self {
            GenEdReq::Set(courses) => {
                codes.extend(courses.iter());
            }
            GenEdReq::SetOpts(course_seqs) => {
                codes.extend(course_seqs.iter().flatten());
            }
            GenEdReq::Courses { courses, .. } | GenEdReq::Credits { courses, .. } => {
                codes.extend(courses.iter());
            }
        }
    }
}
