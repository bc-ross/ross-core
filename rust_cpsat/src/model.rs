//! Model building and constraint logic for the course scheduling solver.

// All model-building logic has been modularized into submodules.
// This file now re-exports the main entry point for model building.

pub use crate::model_context::{ModelBuilderContext, Course};
pub use crate::model_courses::*;
pub use crate::model_prereqs::*;
pub use crate::model_geneds::*;
pub use crate::model_semester::*;

/// Main entry point for building the scheduling model pipeline.
pub use crate::model_context::build_model_pipeline;
