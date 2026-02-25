//! Interactive REPL (Read-Eval-Print Loop) mode.
//!
//! Implements the interactive command-line interface where users can enter
//! commands and see results. The REPL handles variable substitution, command
//! execution, and history management via rustyline.

use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::interpreter::execute::Executor;
use crate::interpreter::{self, Substituter};
use crate::context::Context;

/// Displays a welcome message when entering interactive mode.
fn welcome_message() {
    println!("{}", "Welcome to KeyForge cli".green());
}

/// Runs the interactive REPL mode.
///
/// Enters an interactive loop where:
/// 1. User input is read from stdin (with line editing via rustyline)
/// 2. Variable and command substitutions are resolved
/// 3. Commands are parsed and executed
/// 4. Results are displayed to the user
///
/// # Returns
///
/// `Ok(())` on successful exit (Ctrl+D or exit command)
/// `Err(ReadlineError)` on input errors
///
/// # Loop Behavior
///
/// - `Ctrl+C` (Interrupted) - Prints "CTRL-C" and breaks
/// - `Ctrl+D` (EOF) - Prints "CTRL-D" and breaks
/// - Other errors - Prints error and breaks
pub fn input_loop() -> Result<()> {
    welcome_message();

    // Initialize line editor with history support
    let mut rl = DefaultEditor::new()?;
    // Initialize execution context that persists across commands
    let mut context = Context::new();

    // Initialize executor with all available commands
    let mut executer = Executor::new();
    executer.init_commands();

    loop {
        // Read user input
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                // Step 1: Resolve substitutions ($var and $(cmd))
                match Substituter::resolve(&line, &mut context, &executer) {
                    Ok(resolved_line) => {
                        // Step 2: Parse the resolved line into command tokens
                        match interpreter::parse(&resolved_line) {
                            Ok(args) => {
                                // Step 3: Execute the command with current context
                                match executer.execute(&args, &mut context) {
                                    Ok(output) => {
                                        // Step 4: Display output if non-empty
                                        if !output.is_empty() {
                                            println!("{}", output);
                                        }
                                    }
                                    Err(e) => println!("Error: {}", e),
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break Ok(())
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break Ok(())
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break Ok(())
            }
        }
    }
}
