use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::interpreter::execute::Executor;
use crate::interpreter;

/// show welcome message for cli mode
fn welcome_message() {
    println!("{}", "Welcome to KeyForge cli".green());
}

pub fn input_loop() -> Result<()> {
    welcome_message();

    let mut rl = DefaultEditor::new()?;

    let mut executer = Executor::new();
    executer.init_commands();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                match interpreter::parse(&line) {
                    Ok(args) => {
                        match executer.execute(&args) {
                            Ok(output) => {
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
