//! Command implementations module.
//!
//! This module contains all built-in commands for KeyForge:
//! - `arithmetic` - Math operations (add, sub, mul, div)
//! - `encoding` - Base64 encode/decode
//! - `files` - File I/O (write, read_file)
//! - `random` - Random data generation
//! - `state` - State persistence (save/load)
//! - `utility` - Utility commands (help, clear, exit)
//! - `variables` - Variable management (set, vl, rm)

pub mod console_output;
pub mod arithmetic;
pub mod encoding;
pub mod files;
pub mod random;
pub mod state;
pub mod utility;
pub mod variables;
