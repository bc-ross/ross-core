use anyhow::Result;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
};

use crate::prereqs::CourseReq;
use crate::schedule::Elective;

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub enum GenEdKind {
    Core,
    Foundation,
    SkillAndPerspective,
}

#[derive(Savefile, Serialize, Deserialize, Debug, Clone)]
pub struct GenEd {
    name: String,
    reqs: Elective,
    kind: GenEdKind,
}
