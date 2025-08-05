use crate::{schedule::CourseCode, CC};
use std::collections::HashMap;

pub fn courses() -> HashMap<CourseCode, (String, Option<u32>)> {
    let mut courses = HashMap::new();
    courses.insert(CC!("EENG", 2060), ("English Composition".into(), Some(3)));
    courses.insert(CC!("ENGL", 1010), ("English Composition".into(), Some(3)));
    courses
}
