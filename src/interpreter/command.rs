//! Trait definition for executable commands.
//!
//! The `Command` trait defines the interface that all executable commands must implement.
//! Commands can modify the execution context and return string results that may be
//! printed to console or used in substitutions.

use crate::context::Context;

/// Trait for executable commands in KeyForge.
///
/// All commands must implement this trait, which defines two key methods:
/// - `name()` - returns the command's name (used for command lookup)
/// - `execute()` - performs the command's action and returns a result
///
/// Commands are polymorphic and stored in a `HashMap<String, Box<dyn Command>>`,
/// allowing dynamic dispatch at runtime.
///
/// # Execution Context
///
/// Commands receive a mutable reference to the execution context, which allows them to:
/// - Read existing variables
/// - Create or modify variables
/// - Persist or modify global state
///
/// # Return Values
///
/// All commands return `Result<String, String>`:
/// - `Ok(output)` - successful execution with output to display or use in substitutions
/// - `Err(error)` - execution failed with an error message to display to the user
///
/// The output string is always returned, but printing happens at a higher level.
/// This separation enables both direct output (for `command`) and substitution
/// (for `$(command)`) to work with the same code.
pub trait Command {
    /// Returns the name of the command.
    ///
    /// This name is used for command lookup in the executor's command registry.
    /// Must be lowercase and match the user's input.
    ///
    /// # Returns
    ///
    /// A string slice containing the command name (e.g., "set", "add", "vl").
    fn name(&self) -> &str;

    /// Executes the command with the given arguments in the provided context.
    ///
    /// # Arguments
    ///
    /// * `args` - Command-line arguments (does not include the command name itself)
    /// * `context` - Mutable reference to the execution context for variable access/modification
    ///
    /// # Returns
    ///
    /// - `Ok(String)` - The command output (may be empty string if no output needed)
    /// - `Err(String)` - An error message describing what went wrong
    ///
    /// # Error Handling
    ///
    /// Commands should validate arguments and return descriptive error messages.
    /// Users should see useful feedback about what went wrong (e.g., missing arguments,
    /// invalid types, file not found).
    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String>;
}
