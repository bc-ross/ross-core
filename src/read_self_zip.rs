use lazy_static::lazy_static;

#[path = "../resources/course_reqs/mod.rs"]
mod course_reqs;

#[path = "../resources/programs/mod.rs"]
mod programs;

use crate::schedule::Catalog;

lazy_static! {
    pub static ref CATALOGS: Vec<Catalog> = vec![Catalog {
        geneds: vec![],
        programs: programs::programs(),
        prereqs: course_reqs::prereqs(),
        low_year: 2025,
    }];
}
