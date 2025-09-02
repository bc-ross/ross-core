//! Model building and constraint logic for the course scheduling solver.

mod context;
mod courses;
mod geneds;
mod prereqs;
mod semester;
mod two_stage_schedule;

use context::{ModelBuilderContext, build_model_pipeline};

pub use two_stage_schedule::{generate_multi_schedules, two_stage_lex_schedule};
