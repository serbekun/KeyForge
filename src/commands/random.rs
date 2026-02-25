//! Random data generation commands.
//!
//! Provides commands for generating random values:
//! - `random-num` - Random integer or floating-point number
//! - `random-char` - Random single character
//! - `random-string` - Random string of specified length

use crate::interpreter::Command;
use crate::context::Context;
use rand::Rng;

/// Generates a random number within a specified range.
///
/// Syntax: `random-num <min> <max>`
///
/// If both bounds are integers, returns an integer; otherwise returns a float.
/// Both bounds are inclusive.
pub struct RandomNumCommand;

impl Command for RandomNumCommand {
    fn name(&self) -> &str {
        "random-num"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("random-num requires 2 arguments: min max".to_string());
        }

        let min = resolve_number(&args[0], context)?;
        let max = resolve_number(&args[1], context)?;

        if min > max {
            return Err("min must be <= max".to_string());
        }

        let mut rng = rand::thread_rng();
        
        // Generate integers if both bounds have no fractional part
        if min.fract() == 0.0 && max.fract() == 0.0 {
            let result = rng.gen_range(min as i64..=max as i64);
            Ok(result.to_string())
        } else {
            let result = rng.gen_range(min..=max);
            Ok(result.to_string())
        }
    }
}

/// Generates a single random character (alphanumeric).
///
/// Syntax: `random-char`
///
/// Returns a random character from [a-zA-Z0-9].
pub struct RandomCharCommand;

impl Command for RandomCharCommand {
    fn name(&self) -> &str {
        "random-char"
    }

    fn execute(&self, _args: &[String], _context: &mut Context) -> Result<String, String> {
        let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..chars.len());
        Ok(chars.chars().nth(idx).unwrap().to_string())
    }
}

/// Generates a random string of specified length.
///
/// Syntax: `random-string <len>`
///
/// Generates a string of random alphanumeric characters.
///
/// # Errors
///
/// Returns an error if length is negative or not a valid integer.
pub struct RandomStringCommand;

impl Command for RandomStringCommand {
    fn name(&self) -> &str {
        "random-string"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Err("random-string requires a length argument".to_string());
        }

        let len = resolve_int(&args[0], context)?;

        if len < 0 {
            return Err("Length must be non-negative".to_string());
        }

        let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        let result: String = (0..len)
            .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
            .collect();

        Ok(result)
    }
}

/// Resolves a value to a floating-point number (variable or literal).
fn resolve_number(s: &str, context: &Context) -> Result<f64, String> {
    if let Some(var_name) = s.strip_prefix('$') {
        context
            .get(var_name)
            .ok_or_else(|| format!("Variable '{}' not found", var_name))?
            .as_float()
    } else {
        s.parse::<f64>()
            .map_err(|_| format!("Invalid number: {}", s))
    }
}

/// Resolves a value to an integer (variable or literal).
fn resolve_int(s: &str, context: &Context) -> Result<i64, String> {
    if let Some(var_name) = s.strip_prefix('$') {
        context
            .get(var_name)
            .ok_or_else(|| format!("Variable '{}' not found", var_name))?
            .as_int()
    } else {
        s.parse::<i64>()
            .map_err(|_| format!("Invalid number: {}", s))
    }
}
