use anyhow::Result;
use savefile::prelude::*;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
};

use crate::prereqs::CourseReq;

#[derive(Savefile, Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum CourseCodeSuffix {
    Number(usize),
    Special(String),
    Unique(usize),
}

impl PartialOrd for CourseCodeSuffix {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use CourseCodeSuffix::*;
        match (self, other) {
            (Number(x), Number(y))
            | (Number(x), Unique(y))
            | (Unique(x), Number(y))
            | (Unique(x), Unique(y)) => Some(x.cmp(y)),
            (Special(_), _) | (_, Special(_)) => None,
        }
    }
}

impl From<usize> for CourseCodeSuffix {
    fn from(num: usize) -> Self {
        CourseCodeSuffix::Number(num)
    }
}

impl From<String> for CourseCodeSuffix {
    fn from(s: String) -> Self {
        CourseCodeSuffix::Special(s)
    }
}

impl From<&str> for CourseCodeSuffix {
    fn from(s: &str) -> Self {
        CourseCodeSuffix::Special(s.to_string())
    }
}

impl Display for CourseCodeSuffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CourseCodeSuffix::Number(num) => write!(f, "{}", num),
            CourseCodeSuffix::Special(s) => write!(f, "{}", s),
            CourseCodeSuffix::Unique(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct CourseCode {
    pub stem: String,
    pub code: CourseCodeSuffix,
}

#[macro_export]
// #[macro_use]
macro_rules! CC {
    ($stem:expr, $code:expr) => {
        CourseCode {
            stem: $stem.to_ascii_uppercase(),
            code: $code.into(),
        }
    };
}

impl Display for CourseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.stem, self.code.to_string())
    }
}

pub type Semester = Vec<CourseCode>;

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub enum Elective {
    And(Vec<Elective>),
    Or(Vec<Elective>),
    Courses { num: usize, opts: Vec<CourseCode> },
    Credits { num: usize, opts: Vec<CourseCode> },
    Sequence(Vec<Vec<CourseCode>>),
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub(crate) name: String,
    pub(crate) semesters: Vec<Semester>,
    pub(crate) electives: Vec<Elective>,
    pub(crate) assoc_stems: Vec<String>,
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub enum GenEdKind {
    Core,
    Foundation,
    SkillAndPerspective,
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub struct GenEd {
    name: String,
    reqs: Elective,
    kind: GenEdKind,
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub struct Catalog {
    pub programs: Vec<Program>,
    pub geneds: Vec<GenEd>,
    pub prereqs: HashMap<CourseCode, CourseReq>,
    pub courses: HashMap<CourseCode, (String, Option<u32>)>,
    pub low_year: u32,
}

impl fmt::Display for Catalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "<BC {}-{} Catalog>",
            self.low_year,
            self.low_year + 1
        ))
    }
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub courses: Vec<Semester>,
    pub programs: Vec<String>,
    pub catalog: Catalog,
}

pub fn generate_schedule(programs: Vec<&str>, catalog: Catalog) -> Result<Schedule> {
    // (catalog: )
    let programs: Vec<&Program> = catalog
        .programs
        .iter()
        .filter(|p| programs.contains(&p.name.as_str()))
        .collect();

    let mut combined_semesters: Vec<Semester> = vec![];
    for prog in programs.iter() {
        for (idx, sem) in prog.semesters.iter().enumerate() {
            if let Some(this_sem) = combined_semesters.get_mut(idx) {
                this_sem.extend_from_slice(sem);
            } else {
                combined_semesters.push(sem.clone());
            }
        }
    }

    let mut sched = Schedule {
        courses: combined_semesters,
        programs: programs.iter().map(|x| x.name.to_owned()).collect(),
        catalog,
    };
    sched.reduce()?;
    println!("Is schedule valid? {}", sched.is_valid()?);
    println!(
        "{} different prereq filling options",
        sched.ensure_prereqs()?.len()
    );

    Ok(sched)
}

impl Schedule {
    pub fn reduce<'a>(&'a mut self) -> Result<&'a mut Self> {
        let mut all_codes: HashSet<CourseCode> = HashSet::new();
        self.courses.iter_mut().for_each(|sem| {
            sem.retain(|code| {
                if !all_codes.contains(code) {
                    all_codes.insert(code.clone()); // TODO: is cloning necessary?
                    true
                } else {
                    false
                }
            });
        });
        Ok(self)
    }

    pub fn is_valid(&self) -> Result<bool> {
        Ok(self.are_programs_valid()? && self.validate_prereqs()?)
    }

    fn are_programs_valid(&self) -> Result<bool> {
        let all_sched_codes = self
            .courses
            .iter()
            .flatten()
            .collect::<HashSet<&CourseCode>>();
        Ok(self
            .programs
            .iter()
            .map(|prog_name| {
                let prog = self
                    .catalog
                    .programs
                    .iter()
                    .find(|p| p.name == *prog_name)
                    .ok_or_else(|| anyhow::anyhow!("Program {} not found in catalog", prog_name))?;
                let all_prog_codes = prog
                    .semesters
                    .iter()
                    .flatten()
                    .collect::<HashSet<&CourseCode>>();
                Ok(all_sched_codes.is_superset(&all_prog_codes))
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .all(|x| *x))
    }

    pub fn validate_prereqs(&self) -> Result<bool> {
        for (sem_idx, sem) in self.courses.iter().enumerate() {
            for code in sem {
                let req = self.catalog.prereqs.get(code).unwrap_or(&CourseReq::None);
                if !req.is_satisfied(&self.courses, sem_idx) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    pub fn ensure_prereqs(&self) -> Result<Vec<Self>> {
        let mut unimplemented_prereqs: HashMap<&CourseReq, usize> = HashMap::new();
        for (sem_idx, sem) in self.courses.iter().enumerate() {
            for code in sem {
                let req = self.catalog.prereqs.get(code).unwrap_or(&CourseReq::None);
                if !req.is_satisfied(&self.courses, sem_idx) {
                    unimplemented_prereqs
                        .entry(req)
                        .and_modify(|idx| {
                            if sem_idx > *idx {
                                *idx = sem_idx
                            }
                        })
                        .or_insert(sem_idx);
                    #[cfg(debug_assertions)]
                    println!(
                        "Unimplemented prereq for {}: {:?} at semester {}",
                        code, req, sem_idx
                    );
                }
            }
        }
        println!(
            "{} unimplemented prereqs found",
            unimplemented_prereqs.len()
        );

        Ok(vec![self.clone()])
    }
}
