use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};

use crate::schedule::CourseCode;

#[derive(Savefile, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub enum CourseReq {
    And(Vec<CourseReq>),
    Or(Vec<CourseReq>),
    PreCourse(CourseCode),
    CoCourse(CourseCode),
    Program(String),
    Instructor,
    #[default]
    None,
}
