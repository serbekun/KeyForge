use std::collections::HashMap;
use std::fs::File;
use std::sync::MutexGuard;
use colored::Colorize;
use std::io::Write;
use clear_screen::clear;
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
    resolve_to_string,
    resolve_filename,
    save_state_to_file,
    load_state_from_file,
    encode_base64,
    decode_base64,
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
            if args.len() >= 2 {
                help::show_command_help(&args[1]);
            } else {
                help::show_all_help();
            }
            Ok(String::new())
        }

        "command_list" if !capture_output => {
            help::show_command_list();
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
                                if !res.is_empty() {
                                    results.push(res);
                                }
                            } else {
                                if !res.is_empty() {
                                    println!("{}", res); 
                                }
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
                // Handle variable reference (starts with $)
                if raw_value.starts_with('$') && is_valid_identifier(&raw_value[1..]) {
                    let var_name = &raw_value[1..];
                    let store = get_variable_store().lock().unwrap();
                    
                    // Try to get value from existing variable
                    if let Ok(int_val) = store.get_int_data(var_name) {
                        drop(store);
                        store_parsed_value(name, ParsedValue::Int(int_val), None)?;
                        return Ok(String::new());
                    } else if let Ok(float_val) = store.get_float_data(var_name) {
                        drop(store);
                        store_parsed_value(name, ParsedValue::Float(float_val), None)?;
                        return Ok(String::new());
                    } else if let Ok(string_val) = store.get_string_data(var_name) {
                        drop(store);
                        store_parsed_value(name, ParsedValue::String(string_val), None)?;
                        return Ok(String::new());
                    } else {
                        return Err(format!("Variable {} not found", var_name));
                    }
                }
                
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

            // Substitute variables in the string before printing
            let substituted_string = super::key_forge::substitute_variables_in_string(&raw_value);

            // Use the substituted string instead of raw_value
            if capture_output {
                Ok(substituted_string)
            } else {
                println!("{}", substituted_string);
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
            if args.len() < 3 {
                return Err("Usage: num_to_string <target_variable> <source>".to_string());
            }

            let name = &args[1];
            let raw_value = args[2..].join(" ").trim().to_string();

            let string_val = if raw_value.starts_with("$(") && raw_value.ends_with(')') {
                // Handle command substitution
                let command_content = &raw_value[2..raw_value.len()-1];
                let command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&command_args, true) {
                    Ok(result) => result,
                    Err(e) => return Err(format!("Error executing command: {}", e)),
                }
            } else {
                // Handle direct value or variable reference
                resolve_to_string(&raw_value)?
            };

            // Store the string value using add_data_to_string
            let mut store = get_variable_store().lock().unwrap();
            store.add_data_to_string(name.to_string(), string_val);
            Ok(String::new())
        }

        "push_to_string_back" => {
            if args.len() < 3 {
                return Err("Usage: push_to_string_back <variable_name> <value>".to_string());
            }

            let var_name = &args[1];
            let raw_value = args[2..].join(" ");

            // Handle command substitution
            let value_to_push = if raw_value.starts_with("$(") && raw_value.ends_with(')') {
                let command_content = &raw_value[2..raw_value.len()-1];
                let command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&command_args, true) {
                    Ok(output) => output,
                    Err(e) => return Err(format!("Error executing inner command: {}", e)),
                }
            } else {
                // Check if it's a variable reference
                if is_valid_identifier(&raw_value) {
                    let store = get_variable_store().lock().unwrap();
                    
                    // Try to get value from existing variable
                    if let Ok(int_val) = store.get_int_data(&raw_value) {
                        int_val.to_string()
                    } else if let Ok(float_val) = store.get_float_data(&raw_value) {
                        float_val.to_string()
                    } else if let Ok(string_val) = store.get_string_data(&raw_value) {
                        string_val
                    } else {
                        // If variable doesn't exist, use the raw value
                        raw_value
                    }
                } else {
                    // Direct value
                    raw_value
                }
            };

            // Get the variable store and append to the string
            let mut store = get_variable_store().lock().unwrap();
            
            if let Ok(current_value) = store.get_string_data(var_name) {
                let new_value = current_value + &value_to_push;
                store.add_data_to_string(var_name.to_string(), new_value);
                Ok(String::new())
            } else {
                // If variable doesn't exist, create it with the value
                store.add_data_to_string(var_name.to_string(), value_to_push);
                Ok(String::new())
            }
        }

        "clear" => {
            if capture_output {
                return Err("Command 'clear' cannot be used in variable assignment".to_string());
            }
            
            clear();
            Ok(String::new())
        }

        "if" => {
            if args.len() < 4 {
                return Err("Usage: if <condition> then <command> [else <command>]".to_string());
            }

            // found index "then"
            let then_index: usize = args.iter().position(|arg| arg == "then")
                .ok_or("Expected 'then' after condition".to_string())?;
            
            let condition_parts = &args[1..then_index];
            let condition = condition_parts.join(" ");
            
            // check condition
            let condition_result: bool = super::key_forge::evaluate_condition(&condition)?;
            
            if condition_result {
                // execute then part
                let then_command_start: usize = then_index + 1;
                let else_index = args.iter().position(|arg| arg == "else");
                
                let then_args = if let Some(else_idx) = else_index {
                    &args[then_command_start..else_idx]
                } else {
                    &args[then_command_start..]
                };

                if !then_args.is_empty() {
                    // Support block form: then { ... }
                    let is_block = then_args[0] == "{";
                    if is_block {
                        // Extract block content (between braces) and parse into commands
                        let block_content = if then_args.len() > 1 {
                            then_args[1..then_args.len()-1].join(" ")
                        } else {
                            String::new()
                        };
                        let commands = super::key_forge::parse_block_commands(block_content.trim());
                        for cmd in &commands {
                            let cmd_args = super::key_forge::tokenize_input(cmd);
                            execute_command(&cmd_args, capture_output)?;
                        }
                    } else {
                        execute_command(then_args, capture_output)?;
                    }
                }
            } else {
                // execute else part
                if let Some(else_index) = args.iter().position(|arg| arg == "else") {
                    let else_args = &args[else_index + 1..];
                    if !else_args.is_empty() {
                        // Support block form: else { ... }
                        let is_block = else_args[0] == "{";
                        if is_block {
                            let block_content = if else_args.len() > 1 {
                                else_args[1..else_args.len()-1].join(" ")
                            } else {
                                String::new()
                            };
                            let commands = super::key_forge::parse_block_commands(block_content.trim());
                            for cmd in &commands {
                                let cmd_args = super::key_forge::tokenize_input(cmd);
                                execute_command(&cmd_args, capture_output)?;
                            }
                        } else {
                            execute_command(else_args, capture_output)?;
                        }
                    }
                }
            }
            
            Ok(String::new())
        }

        "while" => {
            if args.len() < 4 {
                return Err("Usage: while <condition> do <command>".to_string());
            }

            let do_index: usize = args.iter().position(|arg| arg == "do")
                .ok_or("Expected 'do' after condition".to_string())?;
            
            let condition_parts = &args[1..do_index];
            let command_args = &args[do_index + 1..];
            
            if command_args.is_empty() {
                return Err("No command specified after 'do'".to_string());
            }

            let is_block = !command_args.is_empty() && command_args[0] == "{";
            let commands = if is_block {
                let block_content = if command_args.len() > 1 {
                    command_args[1..command_args.len()-1].join(" ") // Remove the closing brace
                } else {
                    String::new()
                };
                super::key_forge::parse_block_commands(&block_content)
            } else {
                vec![command_args.join(" ")]
            };
            
            let condition = condition_parts.join(" ");
            
            // Execute the while loop
            loop {
                // Check if we should break
                if super::key_forge::should_break() {
                    super::key_forge::reset_loop_flags();
                    break;
                }
                
                // Check condition
                let condition_result = super::key_forge::evaluate_condition(&condition)?;
                if !condition_result {
                    break;
                }
                
                // Reset continue flag at start of iteration
                super::key_forge::set_continue_flag(false);
                
                // Execute commands in the block
                for cmd in &commands {
                    // Check if we should break or continue
                    if super::key_forge::should_break() {
                        break;
                    }
                    if super::key_forge::should_continue() {
                        break;
                    }
                    
                    let cmd_args = super::key_forge::tokenize_input(cmd);
                    execute_command(&cmd_args, capture_output)?;
                }
                
                // If we hit a break, exit the loop
                if super::key_forge::should_break() {
                    super::key_forge::reset_loop_flags();
                    break;
                }
                
                // Reset continue flag for next iteration
                if super::key_forge::should_continue() {
                    super::key_forge::set_continue_flag(false);
                }
            }
            
            Ok(String::new())
        }

        "for" => {
            if args.len() < 5 {
                return Err("Usage: for <variable> in <start>..<end> do <command>".to_string());
            }

            let var_name = &args[1];
            if args[2] != "in" {
                return Err("Expected 'in' after variable name".to_string());
            }

            let range_str = &args[3];
            if !range_str.contains("..") {
                return Err("Expected range in format start..end".to_string());
            }

            // Find command start: prefer explicit 'do', otherwise look for '{', otherwise take remaining args
            let command_args_slice: &[String] = if let Some(do_idx) = args.iter().position(|a| a == "do") {
                &args[do_idx + 1..]
            } else if let Some(brace_idx) = args.iter().position(|a| a == "{") {
                &args[brace_idx..]
            } else {
                &args[4..]
            };

            let range_parts: Vec<&str> = range_str.split("..").collect();
            if range_parts.len() != 2 {
                return Err("Invalid range format. Use: start..end".to_string());
            }
            
            let start = range_parts[0].parse::<i32>()
                .map_err(|_| "Start must be an integer".to_string())?;
            let end = range_parts[1].parse::<i32>()
                .map_err(|_| "End must be an integer".to_string())?;

            let is_block = !command_args_slice.is_empty() && command_args_slice[0] == "{";
            let commands = if is_block {
                // Extract the content between braces
                // command_args_slice starts with "{"
                let block_content = if command_args_slice.len() > 1 {
                    command_args_slice[1..command_args_slice.len()-1].join(" ") // Join all parts after "{" and before closing "}"
                } else {
                    String::new()
                };
                super::key_forge::parse_block_commands(block_content.trim())
            } else {
                vec![command_args_slice.join(" ")]
            };

            for i in start..end {
                let mut store = super::key_forge::get_variable_store().lock().unwrap();
                store.add_data_to_int(var_name.to_string(), i);
                drop(store);

                for cmd in &commands {
                    if super::key_forge::should_break() {
                        super::key_forge::reset_loop_flags();
                        return Ok(String::new());
                    }
                    if super::key_forge::should_continue() {
                        break;
                    }
                    
                    let cmd_args = super::key_forge::tokenize_input(cmd);
                    execute_command(&cmd_args, capture_output)?;
                }

                if super::key_forge::should_continue() {
                    super::key_forge::set_continue_flag(false);
                }
            }
            
            Ok(String::new())
        }

        "break" => {
            if !capture_output {
                super::key_forge::set_break_flag(true);
                Ok(String::new())
            } else {
                Err("break cannot be used in variable assignment".to_string())
            }
        }

        "continue" => {
            if !capture_output {
                super::key_forge::set_continue_flag(true);
                Ok(String::new())
            } else {
                Err("continue cannot be used in variable assignment".to_string())
            }
        }

        "save_state" => {
            if args.len() < 2 {
                return Err("Usage: save_state <filename>".to_string());
            }

            let filename_raw = &args[1..].join(" ");
            let filename = resolve_filename(filename_raw)?;

            let store = get_variable_store().lock().unwrap();
            save_state_to_file(&filename, &store)?;
            
            if !capture_output {
                println!("State saved to {}", filename);
            }
            Ok(String::new())
        }

        "load_state" => {
            if args.len() < 2 {
                return Err("Usage: load_state <filename>".to_string());
            }

            let filename_raw = &args[1..].join(" ");
            let filename = resolve_filename(filename_raw)?;

            let mut store = get_variable_store().lock().unwrap();
            load_state_from_file(&filename, &mut store)?;
            
            if !capture_output {
                println!("State loaded from {}", filename);
            }
            Ok(String::new())
        }

        "base64_encode" => {
            if args.len() < 2 {
                return Err("Usage: base64_encode <encode string variable name>".to_string());
            }
            
            let store = get_variable_store().lock().unwrap();
            let input = store.get_string_data(&args[1]).map_err(|e| e)?;

            let encode_string = encode_base64(&input);

            if !capture_output {
                println!("{}", encode_string);
                return Ok(String::new());
            } else {
                return Ok(encode_string);
            }
        }

        "base64_decode" => {
            if args.len() < 2 {
                return Err("Usage: base64_decode <decode string variable name>".to_string());
            }

            let store = get_variable_store().lock().unwrap();
            let input = store.get_string_data(&args[1]).map_err(|e| e)?;

            match decode_base64(&input) {
                Ok(decoded) => {
                    if !capture_output {
                        println!("{}", decoded);
                        Ok(String::new())
                    } else {
                        Ok(decoded)
                    }
                }
                Err(e) => Err(e),
            }
        }

        "remove_string_char" => {
            if args.len() < 3 {
                return Err("Usage: remove_string_char <variable_name> <index>".to_string());
            }

            let name = &args[1];
            let index_arg = &args[2];
            let mut store = get_variable_store().lock().unwrap();

            // Handle command substitution for index
            let index_value = if index_arg.starts_with("$(") && index_arg.ends_with(')') {
                let command_content = &index_arg[2..index_arg.len()-1];
                let command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&command_args, true) {
                    Ok(output) => {
                        // Parse the command output as integer
                        output.trim().parse::<i32>()
                            .map_err(|_| "Command output must be a valid integer".to_string())?
                    }
                    Err(e) => return Err(format!("Error executing index command: {}", e)),
                }
            } else {
                // Handle direct value or variable reference for index
                if index_arg.starts_with('$') && is_valid_identifier(&index_arg[1..]) {
                    let var_name = &index_arg[1..];
                    // Try to get integer value from variable
                    if let Ok(int_val) = store.get_int_data(var_name) {
                        int_val
                    } else {
                        return Err(format!("Variable {} not found or not an integer", var_name));
                    }
                } else {
                    // Parse as direct integer value
                    index_arg.parse::<i32>()
                        .map_err(|_| "Index must be a valid integer".to_string())?
                }
            };

            if index_value < 0 {
                return Err("Index cannot be negative".to_string());
            }

            // Get the original string value for output
            // let original_string = store.get_string_data(name)
            //     .map_err(|e| format!("Error getting string variable '{}': {}", name, e))?;

            // Perform the character removal
            match store.remove_string_char(name, index_value as usize) {
                Ok(()) => {
                    if capture_output {
                        // In capture mode, return the modified string
                        store.get_string_data(name)
                            .map_err(|e| format!("Error getting updated string: {}", e))
                    } else {
                        // // In normal mode, print information about the operation
                        // let updated_string = store.get_string_data(name)
                        //     .map_err(|e| format!("Error getting updated string: {}", e))?;
                        // println!("{}", format!("Removed character at index {} from string '{}'", index_value, name).green());
                        // println!("{}: \"{}\"", "Original".yellow(), original_string);
                        // println!("{}: \"{}\"", "Updated".green(), updated_string);
                        Ok(String::new())
                    }
                }
                Err(e) => Err(e),
            }
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