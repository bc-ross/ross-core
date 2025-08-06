use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use std::collections::HashMap;
use std::iter::empty;

mod stem_acct;
mod stem_arch;
mod stem_astr;
mod stem_biol;
mod stem_chem;
mod stem_csci;

pub fn prereqs() -> HashMap<CourseCode, CourseReq> {
    empty()
        .chain(stem_acct::prereqs().into_iter())
        .chain(stem_arch::prereqs().into_iter())
        .chain(stem_astr::prereqs().into_iter())
        .chain(stem_biol::prereqs().into_iter())
        .chain(stem_chem::prereqs().into_iter())
        .chain(stem_csci::prereqs().into_iter())
        .collect()
}
