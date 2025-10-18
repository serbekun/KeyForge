use std::collections::HashMap;
use std::fs::File;
use std::sync::MutexGuard;
use colored::Colorize;
use std::io::Write;
use crate::key_forge::arithmetic::perform_arithmetic;
use crate::key_forge::help;

use super::arithmetic;
use super::key_forge::{
    get_variable_store, 
    parse_value, 
    store_parsed_value, 
    get_random_num, 
    get_random_char, 
    tokenize_input,
    file_mode,
    is_valid_identifier,
    Variables,
    ParsedValue
};

pub fn execute_command(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.is_empty() {
        return Ok(String::new());
    }

    match args[0].as_str() {
        "//" => Ok(String::new()),

        "get_random_num" => {
            if args.len() != 3 {
                return Err(format!("Usage: get_random_num <min> <max>"));
            }

            // Try parsing as int
            if let (Ok(min), Ok(max)) = (args[1].parse::<i32>(), args[2].parse::<i32>()) {
                if min >= max {
                    return Err("min must be less than max".to_string());
                }
                let n: i32 = get_random_num(min, max);
                return if capture_output {
                    Ok(n.to_string())
                } else {
                    println!("{}", n);
                    Ok(String::new())
                };
            }

            // Try parsing as floats
            if let (Ok(min), Ok(max)) = (args[1].parse::<f64>(), args[2].parse::<f64>()) {
                if min >= max {
                    return Err("min must be less than max".to_string());
                }
                let n: f64 = get_random_num(min, max);
                return if capture_output {
                    Ok(n.to_string())
                } else {
                    println!("{}", n);
                    Ok(String::new())
                };
            }

            Err("Arguments must be numbers (integers or floats)".to_string())
        }

        "get_random_char" => {
            let mode = if args.len() == 2 {
                args[1].parse::<i32>().unwrap_or(0)
            } else {
                0
            };

            match get_random_char(mode) {
                Ok(c) => {
                    if capture_output {
                        Ok(c.to_string())
                    } else {
                        println!("{}", c);
                        Ok(String::new())
                    }
                }
                Err(e) => Err(e),
            }
        }

        "quit" | "exit" if !capture_output => {
            if args.len() >= 2 {
                match args[1].parse::<i32>() {
                    Ok(exit_code) => {
                        println!("{}", format!("Program exit with code {}", exit_code).green().bold());
                        std::process::exit(exit_code);
                    } 
                    Err(_) => {
                        println!("{}", "Program exit with code 0".green().bold());
                        std::process::exit(0);
                    }
                }
            }
            println!("{}", "Program exit with code 0".green().bold());
            std::process::exit(0);
        }
        
        "help" if !capture_output => {
            help::show_all_help();
            Ok(String::new())
        }

        "repeat" => {
            if args.len() < 3 {
                return Err("Usage: repeat <count> <command...>".to_string());
            }

            let count: usize = args[1].parse().unwrap_or(0);
            let raw_value = args[2..].join(" ");

            let mut results = Vec::new();

            if raw_value.starts_with("$(") && raw_value.ends_with(')') {
                let command_content = &raw_value[2..raw_value.len()-1];
                let command_args: Vec<String> = tokenize_input(command_content);

                for _ in 0..count {
                    match execute_command(&command_args, true) {
                        Ok(res) => {
                            if capture_output {
                                results.push(res);
                            } else {
                                println!("{}", res); 
                            }
                        }
                        Err(e) => return Err(format!("Error executing inner command: {}", e)),
                    }
                }
            }

            if capture_output {
                Ok(results.join("\n"))
            } else {
                Ok(String::new())
            }
        }

        "set" if !capture_output => {
            if args.len() < 3 {
                return Err("Usage: set <name> <value>".to_string());
            }

            let name = args[1].clone();
            let raw_value = args[2..].join(" ");

            // Check if value starts with $( - command substitution
            if raw_value.starts_with("$(") && raw_value.ends_with(')') {
                let command_content = &raw_value[2..raw_value.len()-1];
                let command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&command_args, true) {
                    Ok(result) => {
                        let parsed_value = parse_value(&result);
                        store_parsed_value(name, parsed_value, Some(&command_args[0]))?;
                        Ok(String::new())
                    }
                    Err(e) => Err(format!("Error executing command: {}", e)),
                }
            } else {
                // Direct value assignment
                let parsed_value = parse_value(&raw_value);
                store_parsed_value(name, parsed_value, None)?;
                Ok(String::new())
            }
        }

        "rm" => {
            let mut store = get_variable_store().lock().unwrap();
            let k = &args[1];

            if store.int_variables.contains_key(k) {
                store.remove_int_data(k);
                return Ok(String::new());
            }

            if store.float_variables.contains_key(k) {
                store.remove_float_data(k);
                return Ok(String::new());
            }

            if store.string_variables.contains_key(k) {
                store.remove_string_data(k);
                return Ok(String::new());
            }

            Err(format!("Variable {} not found", k))
        }

        "print" => {
            if args.len() < 2 {
                return Err("Usage: print <name or literal>".to_string());
            }

            let raw_value = args[1..].join(" ");

            if raw_value.starts_with("$(") && raw_value.ends_with(')') {
                let command_content = &raw_value[2..raw_value.len() - 1];
                let command_args: Vec<String> = tokenize_input(command_content);

                match execute_command(&command_args, capture_output) {
                    Ok(result) => {
                        if capture_output {
                            return Ok(result);
                        } else {
                            println!("{}", result);
                            return Ok(String::new());
                        }
                    }
                    Err(e) => return Err(format!("Error executing command: {}", e)),
                }
            }

            let variable_name: String = args[1].clone();
            let store: MutexGuard<'_, Variables> = get_variable_store().lock().unwrap();

            if let Ok(iv) = store.get_int_data(&variable_name) {
                if capture_output {
                    return Ok(iv.to_string());
                } else {
                    println!("{}", format!("{}", iv).yellow());
                    return Ok(String::new());
                }
            }

            if let Ok(fv) = store.get_float_data(&variable_name) {
                if capture_output {
                    return Ok(fv.to_string());
                } else {
                    println!("{}", format!("{}", fv).yellow());
                    return Ok(String::new());
                }
            }

            if let Ok(sv) = store.get_string_data(&variable_name) {
                if capture_output {
                    return Ok(sv);
                } else {
                    println!("{}", format!("{}", sv).yellow());
                    return Ok(String::new());
                }
            }

            if capture_output {
                Ok(raw_value)
            } else {
                println!("{}" ,format!("{}", raw_value).yellow());
                Ok(String::new())
            }
        }

        "execute_file" => {
            if args.len() < 2 {
                return Err(format!("Usage: execute_file <filename>"));
            }

            file_mode(&args[1]);
            Ok(String::new())
        }

        "vl" => {
            let mode: &str = if args.len() >= 2 { args[1].as_str() } else { "" };
            let store: MutexGuard<'_, Variables> = get_variable_store().lock().unwrap();

            if capture_output {
                use std::fmt::Write;
                let mut output: String = String::new();

                fn collect_section<T: std::fmt::Display>(
                    output: &mut String,
                    title: &str,
                    vars: &HashMap<String, T>,
                    suffix: &str,
                ) {
                    writeln!(output, "{}", title).unwrap();
                    for (k, v) in vars {
                        writeln!(output, "{}: {}{}", k, v, suffix).unwrap();
                    }
                    writeln!(output).unwrap();
                }

                match mode {
                    "i" => collect_section(&mut output, "=== Integer Variables (i32) ===", &store.int_variables, " (i32)"),
                    "f" => collect_section(&mut output, "=== Float Variables (f64) ===", &store.float_variables, " (f64)"),
                    "s" => collect_section(&mut output, "=== String Variables (String) ===", &store.string_variables, " (String)"),
                    _ => {
                        collect_section(&mut output, "=== Integer Variables (i32) ===", &store.int_variables, " (i32)");
                        collect_section(&mut output, "=== Float Variables (f64) ===", &store.float_variables, " (f64)");
                        collect_section(&mut output, "=== String Variables (String) ===", &store.string_variables, " (String)");
                    }
                }

                Ok(output)
            } else {
                store.vl(mode);
                Ok(String::new())
            }
        }

        "to_file" => {
            if args.len() < 3 {
                return Err("Usage: to_file <filename> <command...>".to_string());
            }

            let filename = &args[1];
            let command_args = &args[2..];

            // Handle command substitution
            let raw_command = command_args.join(" ");
            if raw_command.starts_with("$(") && raw_command.ends_with(')') {
                let command_content = &raw_command[2..raw_command.len()-1];
                let inner_command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&inner_command_args, true) {
                    Ok(output) => {
                        use std::fs::OpenOptions;

                        let mut file: File = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(filename)
                            .map_err(|e| format!("Error opening file '{}': {}", filename, e))?;

                        writeln!(file, "{}", output)
                            .map_err(|e| format!("Error writing to file '{}': {}", filename, e))?;
                        
                        println!("{}", format!("Output written to file '{}'", filename).green());
                        Ok(String::new())
                    }
                    Err(e) => return Err(format!("Error executing inner command: {}", e)),
                }
            } else {
                // If it's not a command substitution, execute the command directly
                match execute_command(command_args, true) {
                    Ok(output) => {
                        use std::fs::OpenOptions;

                        let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(filename)
                            .map_err(|e| format!("Error opening file '{}': {}", filename, e))?;

                        writeln!(file, "{}", output)
                            .map_err(|e| format!("Error writing to file '{}': {}", filename, e))?;
                        
                        println!("{}", format!("Output written to file '{}'", filename).green());
                        Ok(String::new())
                    }
                    Err(e) => return Err(format!("Error executing command: {}", e)),
                }
            }
        }

        "add" | "sub" | "mul" | "div" => {
            let command_args = &args[2..];
            let raw_command = command_args.join(" ");
            
            if raw_command.starts_with("$(") && raw_command.ends_with(')') {
                let command_content = &raw_command[2..raw_command.len()-1];
                let inner_command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&inner_command_args, true) {
                    Ok(output) => {
                        let parsed_value = parse_value(&output);
                        arithmetic::perform_arithmetic(&args[0], &args[1], parsed_value)?;
                        Ok(String::new())
                    }
                    Err(e) => Err(format!("Error executing inner command: {}", e)),
                }            
            } else {
                // Parse the value - it could be a direct value or a variable name
                let parsed_value = if is_valid_identifier(&raw_command) {
                    // Try to get value from variable
                    let store = get_variable_store().lock().unwrap();
                    if store.has_variable(&raw_command) {
                        // Get the value from the variable
                        if let Ok(int_val) = store.get_int_data(&raw_command) {
                            ParsedValue::Int(int_val)
                        } else if let Ok(float_val) = store.get_float_data(&raw_command) {
                            ParsedValue::Float(float_val)
                        } else if let Ok(string_val) = store.get_string_data(&raw_command) {
                            ParsedValue::String(string_val)
                        } else {
                            parse_value(&raw_command)
                        }
                    } else {
                        parse_value(&raw_command)
                    }
                } else {
                    parse_value(&raw_command)
                };
                
                perform_arithmetic(&args[0], &args[1], parsed_value)?;
                Ok(String::new())
            }
        }

        "num_to_string" => {

            if args.len() < 2 {
                return Err(format!("Use: <name> <num>"));
            }

            let mut store = get_variable_store().lock().unwrap();
            store.add_data_to_string(args[1].clone(), args[2].clone());

            Ok(String::new())

        }

        _ => {
            if capture_output {
                Err(format!("Command '{}' cannot be used in variable assignment", args[0]))
            } else {
                Err(format!("Unknown command {}", args[0]))
            }
        }
    }
}