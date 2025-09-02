use crate::prereqs::CourseReq;
use crate::schedule::CourseCode;
use std::collections::HashMap;
use std::iter::empty;

mod stem_acct;
mod stem_arch;
mod stem_art;
mod stem_astr;
mod stem_athc;
mod stem_bioc;
mod stem_biol;
mod stem_busi;
mod stem_ceng;
mod stem_chem;
mod stem_civl;
mod stem_crim;
mod stem_csci;
mod stem_danc;
mod stem_econ;
mod stem_eeng;
mod stem_engl;
mod stem_engr;
mod stem_entr;
mod stem_eslg;
mod stem_evca;
mod stem_exsc;
mod stem_finc;
mod stem_fren;
mod stem_grbk;

pub fn prereqs() -> HashMap<CourseCode, CourseReq> {
    empty()
        .chain(stem_acct::prereqs().into_iter())
        .chain(stem_arch::prereqs().into_iter())
        .chain(stem_art::prereqs().into_iter())
        .chain(stem_astr::prereqs().into_iter())
        .chain(stem_athc::prereqs().into_iter())
        .chain(stem_bioc::prereqs().into_iter())
        .chain(stem_biol::prereqs().into_iter())
        .chain(stem_busi::prereqs().into_iter())
        .chain(stem_ceng::prereqs().into_iter())
        .chain(stem_chem::prereqs().into_iter())
        .chain(stem_civl::prereqs().into_iter())
        .chain(stem_crim::prereqs().into_iter())
        .chain(stem_csci::prereqs().into_iter())
        .chain(stem_danc::prereqs().into_iter())
        .chain(stem_econ::prereqs().into_iter())
        .chain(stem_eeng::prereqs().into_iter())
        .chain(stem_engl::prereqs().into_iter())
        .chain(stem_engr::prereqs().into_iter())
        .chain(stem_entr::prereqs().into_iter())
        .chain(stem_eslg::prereqs().into_iter())
        .chain(stem_evca::prereqs().into_iter())
        .chain(stem_exsc::prereqs().into_iter())
        .chain(stem_finc::prereqs().into_iter())
        .chain(stem_fren::prereqs().into_iter())
        .chain(stem_grbk::prereqs().into_iter())
        .collect()
}
