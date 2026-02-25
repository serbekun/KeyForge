//! Base64 encoding/decoding commands.
//!
//! Provides commands for encoding and decoding base64:
//! - `base64-encode` - Encode value to base64
//! - `base64-decode` - Decode base64 value

use crate::interpreter::Command;
use crate::context::Context;
use base64::{engine::general_purpose, Engine as _};

/// Encodes a value to base64.
///
/// Syntax: `base64-encode <value>`
///
/// # Returns
///
/// The base64-encoded representation of the input.
pub struct Base64EncodeCommand;

impl Command for Base64EncodeCommand {
    fn name(&self) -> &str {
        "base64-encode"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Err("base64-encode requires a value argument".to_string());
        }

        let value = if let Some(var_name) = args[0].strip_prefix('$') {
            context
                .get(var_name)
                .ok_or_else(|| format!("Variable '{}' not found", var_name))?
                .as_string()
        } else {
            args[0].clone()
        };

        let encoded = general_purpose::STANDARD.encode(value.as_bytes());
        Ok(encoded)
    }
}

/// Decodes a base64-encoded value.
///
/// Syntax: `base64-decode <value>`
///
/// # Returns
///
/// The decoded string.
///
/// # Errors
///
/// Returns an error if:
/// - The input is not valid base64
/// - The decoded bytes are not valid UTF-8
pub struct Base64DecodeCommand;

impl Command for Base64DecodeCommand {
    fn name(&self) -> &str {
        "base64-decode"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Err("base64-decode requires a value argument".to_string());
        }

        let value = if let Some(var_name) = args[0].strip_prefix('$') {
            context
                .get(var_name)
                .ok_or_else(|| format!("Variable '{}' not found", var_name))?
                .as_string()
        } else {
            args[0].clone()
        };

        let decoded = general_purpose::STANDARD
            .decode(&value)
            .map_err(|e| format!("Failed to decode base64: {}", e))?;

        String::from_utf8(decoded).map_err(|e| format!("Invalid UTF-8 in decoded data: {}", e))
    }
}
