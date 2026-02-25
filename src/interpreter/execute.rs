//! Command execution engine.
//!
//! The executor maintains a registry of all available commands and dispatches
//! command execution based on command names. It also initializes the full set
//! of built-in commands.

use crate::context::Context;
use crate::commands::console_output::{EchoCommand};
use crate::commands::arithmetic::{AddCommand, DivCommand, MulCommand, SubCommand};
use crate::commands::encoding::{Base64DecodeCommand, Base64EncodeCommand};
use crate::commands::files::{ReadFileCommand, WriteCommand};
use crate::commands::random::{RandomCharCommand, RandomNumCommand, RandomStringCommand};
use crate::commands::state::{LoadStateCommand, SaveStateCommand};
use crate::commands::utility::{ClearCommand, ExitCommand, HelpCommand};
use crate::commands::variables::{RmCommand, SetCommand, VlCommand};

use super::command::Command;
use std::collections::HashMap;

/// Executes KeyForge commands.
///
/// The executor maintains a registry of all available commands in a HashMap.
/// Commands are registered by their names and dispatched dynamically at runtime.
///
/// # Example
///
/// ```ignore
/// let mut executor = Executor::new();
/// executor.init_commands();
/// let result = executor.execute(&["add", "5", "3"], &mut context)?;
/// assert_eq!(result, "8");
/// ```
pub struct Executor {
    // Command registry: maps command names to their implementations
    commands: HashMap<String, Box<dyn Command>>,
}

impl Executor {
    /// Creates a new executor with no commands registered.
    ///
    /// # Returns
    ///
    /// An empty executor. Call `init_commands()` to register built-in commands.
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Registers a command in the executor.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to register
    ///
    /// # Notes
    ///
    /// If a command with the same name already exists, it will be replaced.
    pub fn register<C: Command + 'static>(&mut self, command: C) {
        self.commands
            .insert(command.name().to_string(), Box::new(command));
    }

    /// Initializes and registers all built-in commands.
    ///
    /// This must be called before executing any commands. It registers:
    /// - Variable management: set, vl, rm
    /// - Arithmetic: add, sub, mul, div
    /// - Random generation: random-num, random-char, random-string
    /// - File I/O: write, read_file
    /// - Encoding: base64-encode, base64-decode
    /// - State: save-state, load-state
    /// - Utility: clear, help, exit
    pub fn init_commands(&mut self) {
        // Output
        self.register(EchoCommand);

        // Variable management
        self.register(SetCommand);
        self.register(VlCommand);
        self.register(RmCommand);

        // Arithmetic
        self.register(AddCommand);
        self.register(SubCommand);
        self.register(MulCommand);
        self.register(DivCommand);

        // Random generation
        self.register(RandomNumCommand);
        self.register(RandomCharCommand);
        self.register(RandomStringCommand);

        // File I/O
        self.register(WriteCommand);
        self.register(ReadFileCommand);

        // Encoding
        self.register(Base64EncodeCommand);
        self.register(Base64DecodeCommand);

        // State management
        self.register(SaveStateCommand);
        self.register(LoadStateCommand);

        // Utility
        self.register(ClearCommand);
        self.register(ExitCommand);
        self.register(HelpCommand);
    }

    /// Executes a command with the given tokens in the provided context.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Parsed command tokens where first token is command name, rest are arguments
    /// * `context` - Mutable reference to the execution context
    ///
    /// # Returns
    ///
    /// - `Ok(String)` - The command's output (may be empty)
    /// - `Err(String)` - Error message if command not found or execution failed
    ///
    /// # Notes
    ///
    /// The tokens must be pre-parsed. Variable substitution should occur before
    /// calling this method (handled by the substituter in CLI mode).
    pub fn execute(&self, tokens: &[String], context: &mut Context) -> Result<String, String> {
        if tokens.is_empty() {
            return Ok(String::new());
        }

        let cmd = tokens[0].as_str();
        let args = &tokens[1..];

        // Look up command by name and execute it
        match self.commands.get(cmd) {
            Some(command) => command.execute(args, context),
            None => Err(format!("Unknown command: {}", cmd)),
        }
    }
}

