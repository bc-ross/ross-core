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
        .chain(stem_acct::prereqs())
        .chain(stem_arch::prereqs())
        .chain(stem_astr::prereqs())
        .chain(stem_biol::prereqs())
        .chain(stem_chem::prereqs())
        .chain(stem_csci::prereqs())
        .collect()
}
