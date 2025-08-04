use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use std::collections::HashMap;
use std::iter::empty;

mod stem_acct;
mod stem_astr;
mod stem_engl;
mod stem_phys;

pub fn prereqs() -> HashMap<CourseCode, CourseReq> {
    empty()
        .chain(stem_acct::prereqs().into_iter())
        .chain(stem_astr::prereqs().into_iter())
        .chain(stem_engl::prereqs().into_iter())
        .chain(stem_phys::prereqs().into_iter())
        .collect()
}
