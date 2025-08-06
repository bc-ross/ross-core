use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use std::collections::HashMap;
use std::iter::empty;

mod stem_astr;
mod stem_biol;
mod stem_chem;

pub fn prereqs() -> HashMap<CourseCode, CourseReq> {
    empty()
        .chain(stem_astr::prereqs().into_iter())
        .chain(stem_biol::prereqs().into_iter())
        .chain(stem_chem::prereqs().into_iter())
        .collect()
}
