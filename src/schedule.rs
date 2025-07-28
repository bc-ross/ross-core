use anyhow::Result;
use savefile::prelude::*;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub struct CourseCode {
    stem: String,
    code: usize,
}

pub type Semester = Vec<CourseCode>;

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub enum Elective {
    And(Vec<Elective>),
    Or(Vec<Elective>),
    Courses { num: usize, opts: Vec<CourseCode> },
    Credits { num: usize, opts: Vec<CourseCode> },
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub(crate) name: String,
    semesters: Vec<Semester>,
    electives: Vec<Elective>,
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

    Ok(Schedule {
        courses: combined_semesters,
        programs: programs.iter().map(|x| x.name.to_owned()).collect(),
        catalog,
    })
}
