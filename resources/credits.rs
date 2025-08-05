use crate::{CC, schedule::CourseCode};
use std::collections::HashMap;

pub fn credits() -> HashMap<CourseCode, Option<u32>> {
    let mut credits = HashMap::new();
    credits.insert(CC!("ACCT", 1010), Some(3));
    credits.insert(CC!("ASTR", 1010), Some(4));
    credits.insert(CC!("EENG", 1010), Some(3));
    credits.insert(CC!("ENGL", 1010), Some(3));
    credits.insert(CC!("PHYS", 1010), Some(4));
    credits
}
