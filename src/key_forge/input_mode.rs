use std::io::{self, BufRead};
use colored::Colorize;
use std::fs::OpenOptions;
use rustyline::Editor;
use rustyline::error::ReadlineError;

pub fn tokenize_input(input: &str) -> Vec<String> {
let mut parts = Vec::new();
let mut current = String::new();
let mut in_quotes = false;
let mut quote_char = '\0';

for c in input.chars() {
    if in_quotes {
        if c == quote_char {
            in_quotes = false;
            continue;
        }
        current.push(c);
    } else {
        if c == '"' || c == '\'' {
            in_quotes = true;
            quote_char = c;
            continue;
        }
        if c.is_whitespace() {
            if !current.is_empty() {
                parts.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
}

if !current.is_empty() {
    parts.push(current);
}

parts
}

pub fn cli_mode() {
    println!("{}", "KeyForge CLI mode".green());

    let mut rl = Editor::<()>::new().unwrap_or_else(|e| {
        eprintln!("Error init CLI: {}", e);
        std::process::exit(1);
    });

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(input);
                
                let args = tokenize_input(input);
                if args.is_empty() {
                    continue;
                }
                
                match crate::key_forge::execute_command::execute_command(&args, false) {
                    Ok(_) => continue,
                    Err(e) => println!("{}", format!("Error: {}", e).red()),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C - exit");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D - exit");
                break;
            },
            Err(err) => {
                println!("Input error: {:?}", err);
                break;
            }
        }
    }
}

pub fn file_mode(filename: &str) {
    if let Ok(file) = OpenOptions::new().read(true).open(filename) {
        let reader = io::BufReader::new(file);

        // We'll accumulate lines and handle multi-line blocks enclosed in braces { }
        let mut buffer = String::new();
        let mut brace_depth: i32 = 0;

        let mut current_line_number: u32 = 0;


        for line in reader.lines() {
            if let Ok(l) = line {
                current_line_number += 1;

                // debug use
                /*
                println!("interpret line: {}", current_line_number);
                */
                
                // Preserve original line trimming for normal commands but keep spaces inside blocks
                let trimmed = l.trim().to_string();

                // If we're already inside a block, append the raw line (with a space) to buffer
                if brace_depth > 0 {
                    // Use semicolon to separate original lines inside a block so
                    // `parse_block_commands` (which splits on ';') will produce
                    // separate commands for each original line.
                    // Keep spaces around separators so tokenization preserves
                    // `{` and `}` as separate tokens.
                    if buffer.ends_with('{') {
                        // just add a space after the opening brace
                        buffer.push(' ');
                        buffer.push_str(trimmed.as_str());
                    } else {
                        // separate previous command and this one with ' ; '
                        buffer.push_str(" ; ");
                        buffer.push_str(trimmed.as_str());
                    }
                    // Update brace depth based on occurrences in this line
                    brace_depth += trimmed.matches('{').count() as i32;
                    brace_depth -= trimmed.matches('}').count() as i32;

                    if brace_depth <= 0 {
                        // End of block reached; execute the combined command
                        let args = tokenize_input(buffer.trim());
                        let _ = crate::key_forge::execute_command::execute_command(&args, false);
                        buffer.clear();
                        brace_depth = 0;
                    }
                    continue;
                }

                // Not currently in a block. Check if this line starts a block
                if trimmed.contains('{') {
                    // Start collecting block
                    buffer = trimmed.clone();
                    brace_depth += trimmed.matches('{').count() as i32;
                    brace_depth -= trimmed.matches('}').count() as i32;

                    if brace_depth <= 0 {
                        // Opening and closing brace on same line
                        let args = tokenize_input(buffer.trim());
                        match crate::key_forge::execute_command::execute_command(&args, false) {
                            Ok(_) => (),
                            Err(e) => {
                                println!("Error in line {}", current_line_number);
                                println!("{e}");
                                return; // if error stop interpret program
                            }
                        }
                        buffer.clear();
                        brace_depth = 0;
                    }
                    continue;
                }

                // Regular single-line command
                if !trimmed.is_empty() {
                    let args = tokenize_input(&trimmed);
                    match crate::key_forge::execute_command::execute_command(&args, false) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("{}", format!("Error in line {}", current_line_number).red());
                            println!("{}", format!("{}", e).red());
                            return; // if error stop interpret program
                        }
                    }
                }
            }
        }

        // If file ends but buffer still contains something, try to execute it
        if !buffer.trim().is_empty() {
            let args = tokenize_input(buffer.trim());
            let _ = crate::key_forge::execute_command::execute_command(&args, false);
        }
    } else {
        println!("{}", format!("Cannot open file '{}'", filename).red());
    }
}

pub fn interpret_arguments_from_command_line(_args: &[String]) -> Result<(), String> {
    Err("Not implemented".to_string())
}