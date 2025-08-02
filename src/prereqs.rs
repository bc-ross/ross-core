use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};

use crate::schedule::CourseCode;

#[derive(Savefile, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub enum CourseReq {
    And(Vec<CourseReq>),
    Or(Vec<CourseReq>),
    PreCourse(CourseCode),
    CoCourse(CourseCode),
    PreCourseGrade(CourseCode, Grade),
    CoCourseGrade(CourseCode, Grade),
    Program(String),
    Instructor,
    #[default]
    None,
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradeLetter {
    A,
    B,
    C,
    D,
    F,
}

impl PartialOrd for GradeLetter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GradeLetter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8)).reverse()
    }
}

#[derive(Savefile, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub enum GradeQualifier {
    Plus,
    Minus,
    #[default]
    None,
}

impl PartialOrd for GradeQualifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GradeQualifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use GradeQualifier::*;
        match (self, other) {
            (Plus, Plus) | (Minus, Minus) | (None, None) => std::cmp::Ordering::Equal,
            (Plus, _) => std::cmp::Ordering::Greater,
            (_, Plus) => std::cmp::Ordering::Less,
            (Minus, _) => std::cmp::Ordering::Greater,
            (_, Minus) => std::cmp::Ordering::Less,
        }
    }
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Grade {
    pub letter: GradeLetter,
    pub qualifier: GradeQualifier,
}

impl PartialOrd for Grade {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Grade {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.letter
            .cmp(&other.letter)
            .then(self.qualifier.cmp(&other.qualifier))
    }
}

#[macro_export]
macro_rules! GR {
    ($l:ident +) => {
        Grade {
            letter: GradeLetter::$l,
            qualifier: GradeQualifier::Plus,
        }
    };
    ($l:ident -) => {
        Grade {
            letter: GradeLetter::$l,
            qualifier: GradeQualifier::Minus,
        }
    };
    ($l:ident) => {
        Grade {
            letter: GradeLetter::$l,
            qualifier: GradeQualifier::None,
        }
    };
}
