//! Commands for variable management.
//!
//! This module provides commands for:
//! - Creating typed variables (`set`)
//! - Listing all variables (`vl`)
//! - Removing variables (`rm`)

use crate::interpreter::Command;
use crate::context::Context;
use crate::value::Value;

/// Sets a typed variable in the execution context.
///
/// Syntax: `set <type> <name> <value>`
///
/// Creates a new variable or overwrites an existing one with a specific type.
/// The value is parsed according to the specified type.
///
/// # Types
///
/// - `int` - 64-bit signed integer
/// - `float` - 64-bit floating-point number
/// - `string` - UTF-8 text (default for non-numeric values)
/// - `bool` - Boolean value (true/false, 1/0, yes/no)
/// - `binary` - Raw binary data
///
/// # Errors
///
/// Returns an error if:
/// - Less than 3 arguments provided
/// - Unknown type specified
/// - Value cannot be parsed as the specified type
pub struct SetCommand;

impl Command for SetCommand {
    fn name(&self) -> &str {
        "set"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 3 {
            return Err("set requires 3 arguments: type name value".to_string());
        }

        let type_name = &args[0];
        let var_name = &args[1];
        // Join remaining args to support values with spaces
        let value_str = args[2..].join(" ");

        let value = Value::parse(type_name, &value_str)?;
        context.set(var_name.clone(), value);
        Ok(String::new())
    }
}

/// Lists all variables in the execution context.
///
/// Syntax: `vl`
///
/// Displays all currently defined variables with their types and values,
/// sorted alphabetically by name.
pub struct VlCommand;

impl Command for VlCommand {
    fn name(&self) -> &str {
        "vl"
    }

    fn execute(&self, _args: &[String], context: &mut Context) -> Result<String, String> {
        let variables = context.list();
        if variables.is_empty() {
            return Ok("No variables defined".to_string());
        }

        let mut output = String::from("Variables:\n");
        for (name, type_name, value) in variables {
            output.push_str(&format!("  {} ({}): {}\n", name, type_name, value));
        }
        Ok(output.trim_end().to_string())
    }
}

/// Removes a variable from the execution context.
///
/// Syntax: `rm <name>`
///
/// Deletes a variable if it exists. Returns a success message or error
/// if the variable doesn't exist.
pub struct RmCommand;

impl Command for RmCommand {
    fn name(&self) -> &str {
        "rm"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Err("rm requires a variable name".to_string());
        }

        let var_name = &args[0];
        match context.remove(var_name) {
            Some(_) => Ok(format!("Removed variable '{}'", var_name)),
            None => Err(format!("Variable '{}' not found", var_name)),
        }
    }
}
