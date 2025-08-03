use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::empty;
mod stem_acct;
mod stem_arch;

lazy_static! {
    pub static ref PREREQS_MAP: HashMap<&'static CourseCode, &'static CourseReq> = empty()
        .chain(stem_acct::PREREQS.iter().map(|(x, y)| (x, y)))
        .chain(stem_arch::PREREQS.iter().map(|(x, y)| (x, y)))
        .collect();
}
