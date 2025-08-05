use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use std::collections::HashMap;
use std::iter::empty;

mod stem_astr;

pub fn prereqs() -> HashMap<CourseCode, CourseReq> {
    empty().chain(stem_astr::prereqs().into_iter()).collect()
}
