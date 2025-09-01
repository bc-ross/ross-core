use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CourseReasons {
    Foundation { name: String },
    SkillsAndPerspectives { name: String },
    ProgramRequired { prog: String },
    ProgramElective { prog: String, name: String },
}

#[derive(Debug, Clone, Default)]
pub struct ScheduleReasons(pub Rc<RefCell<HashMap<String, Vec<CourseReasons>>>>);
