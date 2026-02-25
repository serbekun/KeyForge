//! Execution context module.
//!
//! This module manages the execution state, storing all variables and their values
//! during program execution. The context is passed through the entire execution
//! pipeline and can be serialized/deserialized for state persistence.

pub mod variables;

pub use variables::Context;
