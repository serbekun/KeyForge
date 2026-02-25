//! Interpreter module.
//!
//! This module implements the core interpretation engine:
//! - `command` - Trait definition for executable commands
//! - `execute` - Command execution engine with command registry
//! - `parser` - Command-line parsing (delegates to shell-words)
//! - `substituter` - Variable and command substitution resolver

pub mod command;
pub mod execute;
pub mod parser;
pub mod substituter;

pub use command::Command;
pub use parser::parse;
pub use substituter::Substituter;
