//! Context struct for model building and shared state.
use cp_sat::builder::{CpModelBuilder, BoolVar, LinearExpr};
use crate::schedule::{Schedule, CourseCode, Catalog};
use crate::prereqs::CourseReq;

#[derive(Clone)]
pub struct Course<'a> {
    pub code: CourseCode,
    pub credits: i64,
    pub required: bool,
    pub geneds: Vec<&'a str>,
    pub elective_group: Option<&'a str>,
    pub prereqs: CourseReq,
}

pub struct ModelBuilderContext<'a> {
    pub model: CpModelBuilder,
    pub vars: Vec<Vec<BoolVar>>,
    pub courses: Vec<Course<'a>>,
    pub num_semesters: usize,
    pub max_credits_per_semester: i64,
    pub min_credits: Option<i64>,
    pub geneds: Option<&'a [crate::geneds::GenEd]>,
    pub catalog: Option<&'a Catalog>,
}
