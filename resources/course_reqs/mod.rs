use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::empty;

mod stem_acct;
mod stem_astr;
mod stem_engl;
mod stem_phys;

lazy_static! {
    pub static ref PREREQS_MAP: HashMap<CourseCode, CourseReq> = empty()
        .chain(stem_acct::prereqs().into_iter())
        .chain(stem_astr::prereqs().into_iter())
        .chain(stem_engl::prereqs().into_iter())
        .chain(stem_phys::prereqs().into_iter())
        .collect();
}
