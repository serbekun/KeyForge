//! File I/O commands.
//!
//! Provides commands for reading and writing files:
//! - `write` - Write content to file
//! - `read_file` - Read file contents

use crate::interpreter::Command;
use crate::context::Context;
use std::fs::OpenOptions;
use std::io::Write;

/// Writes content to a file.
///
/// Syntax: `write <filename> <content> <mode>`
///
/// # Modes
///
/// - `w` - Write (overwrite existing file)
/// - `a` - Append to existing file
///
/// # Errors
///
/// Returns an error if:
/// - File cannot be opened or written
/// - Invalid mode specified
pub struct WriteCommand;

impl Command for WriteCommand {
    fn name(&self) -> &str {
        "write"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 3 {
            return Err("write requires 3 arguments: filename content mode".to_string());
        }

        let filename = &args[0];
        let content_arg = &args[1];
        let mode = &args[2];

        // Resolve the content (may be a variable reference)
        let content = if let Some(var_name) = content_arg.strip_prefix('$') {
            context
                .get(var_name)
                .ok_or_else(|| format!("Variable '{}' not found", var_name))?
                .as_string()
        } else {
            content_arg.clone()
        };

        // Open file with appropriate mode
        let mut file = match mode.as_str() {
            "w" => OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(filename)
                .map_err(|e| format!("Failed to open file: {}", e))?,
            "a" => OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(filename)
                .map_err(|e| format!("Failed to open file: {}", e))?,
            _ => return Err(format!("Invalid mode '{}'. Use 'w' or 'a'", mode)),
        };

        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(format!("Wrote {} bytes to '{}'", content.len(), filename))
    }
}

/// Reads and returns the contents of a file.
///
/// Syntax: `read_file <filename>`
///
/// # Returns
///
/// The entire file contents as a string.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub struct ReadFileCommand;

impl Command for ReadFileCommand {
    fn name(&self) -> &str {
        "read_file"
    }

    fn execute(&self, args: &[String], _context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Err("read_file requires a filename argument".to_string());
        }

        let filename = &args[0];
        std::fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))
    }
}
