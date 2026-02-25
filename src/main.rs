//! KeyForge - Command-line interpreter for data generation and variable management.
//!
//! KeyForge provides a rich CLI environment with support for:
//! - Typed variable management (int, float, string, bool, binary)
//! - Arithmetic and string operations
//! - Random data generation
//! - File I/O operations
//! - Base64 encoding/decoding
//! - State persistence (save/load to JSON)
//! - Variable substitution syntax ($var, $(command))
//!
//! # Architecture
//!
//! The system consists of:
//! - **Core**: Value type system with automatic type conversions
//! - **Context**: Manages variable storage and state
//! - **Interpreter**: Parses and executes commands with substitution
//! - **Commands**: Modular command implementations
//! - **CLI**: Interactive REPL for user interaction
//!
//! # Example Usage
//!
//! ```ignore
//! set int x 10
//! set int y 5
//! add $x $y
//! write file.txt $(random-string 10) w
//! ```

use std::env;

pub mod cli_mode;
pub mod commands;
pub mod context;
pub mod interpreter;
pub mod value;

use crate::cli_mode::input_loop::input_loop;

/// Entry point for the KeyForge interpreter.
///
/// If no command-line arguments are provided, enters interactive REPL mode.
/// Future versions may support script files or direct command execution.
fn main() {
    let args: Vec<String> = env::args().collect();

    // If no arguments provided, start interactive mode
    if args.len() <= 1 {
        let _ = input_loop();
    }
}