use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::schedule::CourseCode;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CourseReasons {
    Core { name: String },
    Foundation { name: String },
    SkillsAndPerspective { name: String },
    ProgramRequired { prog: String },
    ProgramElective { prog: String, name: String },
    CourseReq { course: CourseCode },
}

#[derive(Debug, Clone, Default)]
pub struct ScheduleReasons(pub Rc<RefCell<HashMap<CourseCode, Vec<CourseReasons>>>>);
