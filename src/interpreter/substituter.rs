//! Variable and command substitution engine.
//!
//! The substituter processes strings containing substitution syntax:
//! - `$variable_name` - replaced with the variable's string value
//! - `$(command args...)` - replaced with the command's output
//!
//! This module handles the parsing and resolution of these substitutions,
//! enabling dynamic string generation and command composition.

use crate::context::Context;
use crate::interpreter::execute::Executor;
use crate::interpreter::parse;

/// Handles substitution of variables and commands in strings.
///
/// The substituter processes input strings and replaces substitution patterns
/// with their resolved values. It must have access to the context (for variables)
/// and the executor (for command invocation).
pub struct Substituter;

impl Substituter {
    /// Resolves all substitutions in an input string.
    ///
    /// Processes the string character by character, looking for substitution patterns:
    /// - `$identifier` - variable reference
    /// - `$(...)` - command invocation with balanced parentheses
    ///
    /// # Arguments
    ///
    /// * `input` - The string containing potential substitution patterns
    /// * `context` - The execution context for variable lookup
    /// * `executor` - The command executor for running commands
    ///
    /// # Returns
    ///
    /// - `Ok(String)` - The input with all substitutions resolved
    /// - `Err(String)` - An error if a variable is not found or command execution fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let resolved = Substituter::resolve("Hello $name", &mut ctx, &executor)?;
    /// // If name="World", resolves to "Hello World"
    ///
    /// let resolved = Substituter::resolve("$(add 2 3)", &mut ctx, &executor)?;
    /// // Resolves to "5"
    /// ```
    ///
    /// # Notes
    ///
    /// - Substitutions are not recursive (inner `$(...)` inside command args are literal)
    /// - Variable names must contain only alphanumeric characters and underscores
    /// - Unmatched parentheses in `$(...)` produce an error
    pub fn resolve(input: &str, context: &mut Context, executor: &Executor) -> Result<String, String> {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' {
                if chars.peek() == Some(&'(') {
                    // Command substitution: $(...)
                    chars.next(); // consume '('
                    let cmd_str = Self::extract_until(&mut chars, ')')?;
                    let tokens = parse(&cmd_str)?;
                    // Execute the command and capture its output
                    let cmd_result = executor.execute(&tokens, context)?;
                    result.push_str(&cmd_result);
                } else {
                    // Variable substitution: $identifier
                    let var_name = Self::extract_identifier(&mut chars);
                    if var_name.is_empty() {
                        result.push('$');
                    } else {
                        match context.get(&var_name) {
                            Some(value) => result.push_str(&value.to_string_value()),
                            None => return Err(format!("Variable '{}' not found", var_name)),
                        }
                    }
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    /// Extracts text until a closing delimiter, handling nested delimiters.
    ///
    /// Used internally to extract the content of `$(...)` constructs.
    /// Tracks nesting depth to handle balanced parentheses.
    ///
    /// # Arguments
    ///
    /// * `chars` - Iterator over remaining input characters
    /// * `end` - The delimiter to stop at (e.g., ')')
    ///
    /// # Returns
    ///
    /// - `Ok(String)` - The extracted text (not including the closing delimiter)
    /// - `Err(String)` - If the closing delimiter is never found
    fn extract_until(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        end: char,
    ) -> Result<String, String> {
        let mut result = String::new();
        let mut depth = 0; // Track nested parentheses

        while let Some(ch) = chars.next() {
            if ch == '(' {
                depth += 1;
                result.push(ch);
            } else if ch == end && depth == 0 {
                // Found closing delimiter at correct nesting level
                return Ok(result);
            } else {
                if ch == ')' && depth > 0 {
                    depth -= 1;
                }
                result.push(ch);
            }
        }

        Err(format!("Unclosed substitution: expected '{}'", end))
    }

    /// Extracts a variable name from the character iterator.
    ///
    /// Variable names consist of alphanumeric characters and underscores.
    /// Stops when encountering other characters (which remain in the iterator).
    ///
    /// # Arguments
    ///
    /// * `chars` - Iterator over remaining input characters (modified in place)
    ///
    /// # Returns
    ///
    /// The extracted variable name (may be empty if the next character is not valid).
    fn extract_identifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut result = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        result
    }
}
