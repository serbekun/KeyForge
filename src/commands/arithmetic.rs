//! Arithmetic and string operations.
//!
//! This module provides commands for mathematical operations and string manipulation:
//! - `add` - Addition or concatenation
//! - `sub` - Subtraction
//! - `mul` - Multiplication
//! - `div` - Division

use crate::interpreter::Command;
use crate::context::Context;
use crate::value::Value;

/// Adds two values or concatenates strings.
///
/// Syntax: `add <val1> <val2>`
///
/// # Behavior
///
/// If both values are numeric (or numeric strings), performs addition.
/// Otherwise, concatenates the string representations.
///
/// # Type Detection
///
/// Variables are resolved with their types. String literals are tested
/// to see if they can be parsed as numbers. If both operands are numeric,
/// addition is performed; otherwise concatenation occurs.
///
/// # Examples
///
/// - `add 5 3` → `8`
/// - `add 5 $(add 2 3)` → `10` (command result is numeric)
/// - `add Hello_ world` → `Hello_world`
/// - `add $greeting $name` → concatenates the variables
pub struct AddCommand;

impl Command for AddCommand {
    fn name(&self) -> &str {
        "add"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("add requires 2 arguments".to_string());
        }

        let val1_str = &args[0];
        let val2_str = &args[1];

        let val1 = resolve_value(val1_str, context)?;
        let val2 = resolve_value(val2_str, context)?;

        // Determine if we should do numeric addition or string concatenation
        // Variables maintain their types, but string literals are checked if numeric
        let is_val1_numeric = matches!(&val1, Value::Int(_) | Value::Float(_))
            || (matches!(&val1, Value::String(s) if is_numeric_string(s)));
        let is_val2_numeric = matches!(&val2, Value::Int(_) | Value::Float(_))
            || (matches!(&val2, Value::String(s) if is_numeric_string(s)));

        if is_val1_numeric && is_val2_numeric {
            // Both values are numeric: perform addition
            let n1 = val1.as_float()?;
            let n2 = val2.as_float()?;
            Ok((n1 + n2).to_string())
        } else {
            // At least one value is non-numeric: concatenate as strings
            let s1 = val1.as_string();
            let s2 = val2.as_string();
            Ok(format!("{}{}", s1, s2))
        }
    }
}

/// Subtracts two numeric values.
///
/// Syntax: `sub <val1> <val2>`
///
/// Performs `val1 - val2`. Both values must be convertible to numbers.
///
/// # Errors
///
/// Returns an error if values cannot be converted to numbers.
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "sub"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("sub requires 2 arguments".to_string());
        }

        let val1 = resolve_value(&args[0], context)?;
        let val2 = resolve_value(&args[1], context)?;

        let n1 = val1.as_float()?;
        let n2 = val2.as_float()?;
        Ok((n1 - n2).to_string())
    }
}

/// Multiplies two numeric values.
///
/// Syntax: `mul <val1> <val2>`
///
/// Performs `val1 * val2`. Both values must be convertible to numbers.
pub struct MulCommand;

impl Command for MulCommand {
    fn name(&self) -> &str {
        "mul"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("mul requires 2 arguments".to_string());
        }

        let val1 = resolve_value(&args[0], context)?;
        let val2 = resolve_value(&args[1], context)?;

        let n1 = val1.as_float()?;
        let n2 = val2.as_float()?;
        Ok((n1 * n2).to_string())
    }
}

/// Divides two numeric values.
///
/// Syntax: `div <val1> <val2>`
///
/// Performs `val1 / val2`. Both values must be convertible to numbers.
///
/// # Errors
///
/// Returns an error if:
/// - Values cannot be converted to numbers
/// - Attempting to divide by zero
pub struct DivCommand;

impl Command for DivCommand {
    fn name(&self) -> &str {
        "div"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("div requires 2 arguments".to_string());
        }

        let val1 = resolve_value(&args[0], context)?;
        let val2 = resolve_value(&args[1], context)?;

        let n1 = val1.as_float()?;
        let n2 = val2.as_float()?;

        if n2 == 0.0 {
            return Err("Division by zero".to_string());
        }

        Ok((n1 / n2).to_string())
    }
}

/// Resolves a value from either a variable or a string literal.
///
/// # Arguments
///
/// * `s` - The input string, which may start with '$' for variable substitution
/// * `context` - The execution context for variable lookup
///
/// # Returns
///
/// A Value object. If `s` starts with '$', looks up the variable;
/// otherwise treats `s` as a string literal.
fn resolve_value(s: &str, context: &Context) -> Result<Value, String> {
    if let Some(var_name) = s.strip_prefix('$') {
        context
            .get(var_name)
            .ok_or_else(|| format!("Variable '{}' not found", var_name))
    } else {
        Ok(Value::String(s.to_string()))
    }
}

/// Help function that checks if a string represents a valid number.
///
/// Returns true if the string can be parsed as a floating-point number.
fn is_numeric_string(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}
