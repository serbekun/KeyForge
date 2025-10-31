use crate::key_forge::arithmetic::perform_arithmetic;
use crate::key_forge::help;
use clear_screen::clear;
use colored::Colorize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::MutexGuard;

use super::arithmetic;
use super::{
    //expression,
    key_forge::{
        decode_base64, encode_base64, file_mode, get_random_char, get_random_num,
        get_variable_store, is_valid_identifier, load_state_from_file, parse_value,
        resolve_filename, resolve_to_string, save_state_to_file, store_parsed_value,
        tokenize_input, value_to_string, write_to_file_with_mode, read_from_file, ParsedValue, Variables,
    },
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
                        println!(
                            "{}",
                            format!("Program exit with code {}", exit_code)
                                .green()
                                .bold()
                        );
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

            // Parse count - can be variable, command substitution, or direct number
            let count_raw = &args[1];
            let count = if count_raw.starts_with("$(") && count_raw.ends_with(')') {
                // Handle command substitution for count
                let command_content = &count_raw[2..count_raw.len() - 1];
                let command_args: Vec<String> = tokenize_input(command_content);

                match execute_command(&command_args, true) {
                    Ok(output) => output.trim().parse::<usize>().map_err(|_| {
                        "Command output must be a valid positive integer".to_string()
                    })?,
                    Err(e) => return Err(format!("Error executing count command: {}", e)),
                }
            } else if count_raw.starts_with('$') && is_valid_identifier(&count_raw[1..]) {
                // Handle variable for count
                let store = get_variable_store().lock().unwrap();
                let var_name = &count_raw[1..];

                if let Ok(int_val) = store.get_int_data(var_name) {
                    if int_val < 0 {
                        return Err("Count cannot be negative".to_string());
                    }
                    int_val as usize
                } else {
                    return Err(format!(
                        "Variable '{}' not found or not an integer",
                        var_name
                    ));
                }
            } else {
                // Handle direct number
                count_raw
                    .parse::<usize>()
                    .map_err(|_| "Count must be a valid positive integer".to_string())?
            };

            let raw_command = args[2..].join(" ");

            let mut results = Vec::new();

            if raw_command.starts_with("$(") && raw_command.ends_with(')') {
                let command_content = &raw_command[2..raw_command.len() - 1];
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
            } else {
                // Execute the command directly (not as substitution)
                let command_args: Vec<String> = tokenize_input(&raw_command);

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
                return Err("Usage: set <n> <value>".to_string());
            }

            let name = args[1].clone();
            let raw_value = args[2..].join(" ");

            // Use our new expression evaluator that handles variables, commands and arrays
            match crate::key_forge::expression::evaluate_expression(&raw_value) {
                Ok(parsed_value) => {
                    store_parsed_value(name, parsed_value, None)?;
                    Ok(String::new())
                }
                Err(e) => Err(format!("Error evaluating expression: {}", e)),
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
            let mode: &str = if args.len() >= 2 {
                args[1].as_str()
            } else {
                ""
            };
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
                    "i" => collect_section(
                        &mut output,
                        "=== Integer Variables (i32) ===",
                        &store.int_variables,
                        " (i32)",
                    ),
                    "f" => collect_section(
                        &mut output,
                        "=== Float Variables (f64) ===",
                        &store.float_variables,
                        " (f64)",
                    ),
                    "s" => collect_section(
                        &mut output,
                        "=== String Variables (String) ===",
                        &store.string_variables,
                        " (String)",
                    ),
                    _ => {
                        collect_section(
                            &mut output,
                            "=== Integer Variables (i32) ===",
                            &store.int_variables,
                            " (i32)",
                        );
                        collect_section(
                            &mut output,
                            "=== Float Variables (f64) ===",
                            &store.float_variables,
                            " (f64)",
                        );
                        collect_section(
                            &mut output,
                            "=== String Variables (String) ===",
                            &store.string_variables,
                            " (String)",
                        );
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
                let command_content = &raw_command[2..raw_command.len() - 1];
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
                let command_content = &raw_command[2..raw_command.len() - 1];
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
                let command_content = &raw_value[2..raw_value.len() - 1];
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
                let command_content = &raw_value[2..raw_value.len() - 1];
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
            let then_index: usize = args
                .iter()
                .position(|arg| arg == "then")
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
                            then_args[1..then_args.len() - 1].join(" ")
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
                                else_args[1..else_args.len() - 1].join(" ")
                            } else {
                                String::new()
                            };
                            let commands =
                                super::key_forge::parse_block_commands(block_content.trim());
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

            let do_index: usize = args
                .iter()
                .position(|arg| arg == "do")
                .ok_or("Expected 'do' after condition".to_string())?;

            let condition_parts = &args[1..do_index];
            let command_args = &args[do_index + 1..];

            if command_args.is_empty() {
                return Err("No command specified after 'do'".to_string());
            }

            let is_block = !command_args.is_empty() && command_args[0] == "{";
            let commands = if is_block {
                let block_content = if command_args.len() > 1 {
                    command_args[1..command_args.len() - 1].join(" ") // Remove the closing brace
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
            let command_args_slice: &[String] =
                if let Some(do_idx) = args.iter().position(|a| a == "do") {
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

            let start = range_parts[0]
                .parse::<i32>()
                .map_err(|_| "Start must be an integer".to_string())?;
            let end = range_parts[1]
                .parse::<i32>()
                .map_err(|_| "End must be an integer".to_string())?;

            let is_block = !command_args_slice.is_empty() && command_args_slice[0] == "{";
            let commands = if is_block {
                // Extract the content between braces
                // command_args_slice starts with "{"
                let block_content = if command_args_slice.len() > 1 {
                    command_args_slice[1..command_args_slice.len() - 1].join(" ")
                // Join all parts after "{" and before closing "}"
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
                let command_content = &index_arg[2..index_arg.len() - 1];
                let command_args: Vec<String> = tokenize_input(command_content);

                match execute_command(&command_args, true) {
                    Ok(output) => {
                        // Parse the command output as integer
                        output
                            .trim()
                            .parse::<i32>()
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
                    index_arg
                        .parse::<i32>()
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
                        store
                            .get_string_data(name)
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

        "push" => {
            if args.len() < 3 {
                return Err("Usage: push <array_name> <value>".to_string());
            }

            let array_name = &args[1];
            let value_str = &args[2..].join(" ");

            // Use our new expression evaluator for the value to push
            let parsed_value = super::expression::evaluate_expression(value_str)?;

            let mut store = get_variable_store().lock().unwrap();

            if let Ok(mut array) = store.get_array_data(array_name) {
                array.push(parsed_value);
                store.add_data_to_array(array_name.to_string(), array);
                Ok(String::new())
            } else {
                // Create new array if it doesn't exist
                let new_array = vec![parsed_value];
                store.add_data_to_array(array_name.to_string(), new_array);
                Ok(String::new())
            }
        }

        "pop" => {
            if args.len() < 2 {
                return Err("Usage: pop <array_name>".to_string());
            }

            let array_name = &args[1];
            let mut store = get_variable_store().lock().unwrap();

            if let Ok(mut array) = store.get_array_data(array_name) {
                if let Some(popped_value) = array.pop() {
                    store.add_data_to_array(array_name.to_string(), array);
                    let result = value_to_string(&popped_value);
                    if capture_output {
                        Ok(result)
                    } else {
                        println!("{}", result);
                        Ok(String::new())
                    }
                } else {
                    Err("Array is empty".to_string())
                }
            } else {
                Err(format!("Array '{}' not found", array_name))
            }
        }

        "len" => {
            if args.len() < 2 {
                return Err("Usage: len <variable_name>".to_string());
            }

            let var_name = &args[1];
            let store = get_variable_store().lock().unwrap();

            let length = if let Ok(array) = store.get_array_data(var_name) {
                array.len()
            } else if let Ok(dict) = store.get_dict_data(var_name) {
                dict.len()
            } else if let Ok(string) = store.get_string_data(var_name) {
                string.chars().count()
            } else {
                return Err(format!(
                    "Variable '{}' not found or not a collection/string",
                    var_name
                ));
            };

            if capture_output {
                Ok(length.to_string())
            } else {
                println!("{}", length);
                Ok(String::new())
            }
        }

        "keys" => {
            if args.len() < 2 {
                return Err("Usage: keys <dict_name>".to_string());
            }

            let dict_name = &args[1];
            let store = get_variable_store().lock().unwrap();

            if let Ok(dict) = store.get_dict_data(dict_name) {
                let keys: Vec<String> = dict.keys().cloned().collect();
                let result = format!("[{}]", keys.join(", "));

                if capture_output {
                    Ok(result)
                } else {
                    println!("{}", result);
                    Ok(String::new())
                }
            } else {
                Err(format!("Dictionary '{}' not found", dict_name))
            }
        }

        "values" => {
            if args.len() < 2 {
                return Err("Usage: values <dict_name>".to_string());
            }

            let dict_name = &args[1];
            let store = get_variable_store().lock().unwrap();

            if let Ok(dict) = store.get_dict_data(dict_name) {
                let values: Vec<String> = dict.values().map(value_to_string).collect();
                let result = format!("[{}]", values.join(", "));

                if capture_output {
                    Ok(result)
                } else {
                    println!("{}", result);
                    Ok(String::new())
                }
            } else {
                Err(format!("Dictionary '{}' not found", dict_name))
            }
        }

        "get" => {
            if args.len() < 3 {
                return Err("Usage: get <collection_name> <key/index>".to_string());
            }

            let collection_name = &args[1];
            let key_str = &args[2];
            let store = get_variable_store().lock().unwrap();

            // Try as array first
            if let Ok(array) = store.get_array_data(collection_name) {
                let index: usize = key_str
                    .parse()
                    .map_err(|_| "Array index must be a non-negative integer".to_string())?;

                if index < array.len() {
                    let result = value_to_string(&array[index]);
                    if capture_output {
                        Ok(result)
                    } else {
                        println!("{}", result);
                        Ok(String::new())
                    }
                } else {
                    Err(format!(
                        "Index {} out of bounds for array '{}'",
                        index, collection_name
                    ))
                }
            }
            // Try as dictionary
            else if let Ok(dict) = store.get_dict_data(collection_name) {
                if let Some(value) = dict.get(key_str) {
                    let result = value_to_string(value);
                    if capture_output {
                        Ok(result)
                    } else {
                        println!("{}", result);
                        Ok(String::new())
                    }
                } else {
                    Err(format!(
                        "Key '{}' not found in dictionary '{}'",
                        key_str, collection_name
                    ))
                }
            } else {
                Err(format!("Collection '{}' not found", collection_name))
            }
        }

        "set" => {
            if args.len() < 4 {
                return Err("Usage: set <collection_name> <key/index> <value>".to_string());
            }

            let collection_name = &args[1];
            let key_str = &args[2];
            let value_str = &args[3..].join(" ");

            let parsed_value = if value_str.starts_with("$(") && value_str.ends_with(')') {
                let command_content = &value_str[2..value_str.len() - 1];
                let command_args: Vec<String> = tokenize_input(command_content);

                match execute_command(&command_args, true) {
                    Ok(output) => parse_value(&output),
                    Err(e) => return Err(format!("Error executing inner command: {}", e)),
                }
            } else {
                parse_value(value_str)
            };

            let mut store = get_variable_store().lock().unwrap();

            // Try as array first
            if let Ok(mut array) = store.get_array_data(collection_name) {
                let index: usize = key_str
                    .parse()
                    .map_err(|_| "Array index must be a non-negative integer".to_string())?;

                if index < array.len() {
                    array[index] = parsed_value;
                    store.add_data_to_array(collection_name.to_string(), array);
                    Ok(String::new())
                } else {
                    Err(format!(
                        "Index {} out of bounds for array '{}'",
                        index, collection_name
                    ))
                }
            }
            // Try as dictionary
            else if let Ok(mut dict) = store.get_dict_data(collection_name) {
                dict.insert(key_str.to_string(), parsed_value);
                store.add_data_to_dict(collection_name.to_string(), dict);
                Ok(String::new())
            } else {
                Err(format!("Collection '{}' not found", collection_name))
            }
        }

        "multi_arg_command" => {
            if args.len() < 2 {
                return Err("Usage: multi_arg_command <arg1> <arg2> ...".to_string());
            }

            let processed_args = &args[1..].join(" ");

            for (i, arg) in processed_args.chars().enumerate() {
                println!("Arg {}: {}", i + 1, arg);
            }

            Ok(String::new())

        }

        "write_file" => {
            if args.len() < 3 {
                return Err(String::from("Usage: write_file <filename> <content> <append>"));
            }

            let filename = &args[1];
            let content = &args[2];
            let append = &args[3];

            println!("filename: {}", filename);
            println!("content: {}", content);
            println!("append: {}", append);

            let filename = if filename.starts_with("$(") && filename.ends_with(')') {
                let command_content = &filename[2..filename.len() - 1];
                let command_args: Vec<String> = tokenize_input(command_content);

                match execute_command(&command_args, true) {
                    Ok(output) => output,
                    Err(e) => return Err(e),
                }
            } else if filename.starts_with('$') {
                let var_name = &filename[1..];
                let store = get_variable_store().lock().unwrap();
                match store.get_string_data(var_name) {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Undefined variable: {}", e)),
                }
            } else {
                filename.to_string()
            };

            let content = if content.starts_with("$(") && content.ends_with(')') {
                let command_content = &&content[2..content.len() - 1];
                let command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&command_args, true) {
                    Ok(output) => output,
                    Err(e) => return Err(e),
                }
            } else if content.starts_with('$') {
                let var_name = &content[1..];
                let store = get_variable_store().lock().unwrap();
                match store.get_string_data(var_name) {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Undefined variable: {}", e)),
                }
            } else {
                content.to_string()
            };
            
            // FIXED: Use 'append' variable instead of 'content'
            let append = if append.starts_with("$(") && append.ends_with(')') {
                let command_content = &append[2..append.len() - 1];
                let command_args: Vec<String> = tokenize_input(command_content);

                match execute_command(&command_args, true) {
                    Ok(output) => output,
                    Err(e) => return Err(e),
                }
            } else if append.starts_with('$') {
                let var_name = &append[1..]; // FIXED: Use append instead of content
                let store = get_variable_store().lock().unwrap();
                match store.get_string_data(var_name) {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Undefined variable: {}", e)),
                }
            } else {
                append.to_string()
            };
            
            let should_append: bool = if append == "a" {
                true
            } else if append == "w" {
                false
            } else {
                return Err(format!("Unknown mode for write_file '{}' use can use only 'w' or 'a'", append));
            };

            let result = write_to_file_with_mode(&filename, &content, should_append);

            match result {
                Ok(()) => Ok(String::new()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        Err(String::from("Permission denied"))
                    },
                    std::io::ErrorKind::NotFound => {
                        Err(String::from("Directory does not exist"))
                    },
                    std::io::ErrorKind::AlreadyExists => {
                        Err(String::from("File exist but blocked"))
                    },
                    _ => Err(String::from("Error write file"))
                },
            }
        }

        "read_file" => {
            if args.len() < 2 {
                return Err(String::from("Usage: read_file <filename>"));
            }

            let filename = &args[1];

            println!("filename: {}", filename);

            let filename = if filename.starts_with("$(") && filename.ends_with(')') {
                let command_content = &filename[2..filename.len() - 1];
                let command_args: Vec<String> = tokenize_input(command_content);

                match execute_command(&command_args, true) {
                    Ok(output) => output,
                    Err(e) => return Err(e),
                }
            } else if filename.starts_with('$') {
                let var_name = &filename[1..];
                let store = get_variable_store().lock().unwrap();
                match store.get_string_data(var_name) {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Undefined variable: {}", e)),
                }
            } else {
                filename.to_string()
            };

            let result = read_from_file(&filename);

            match result {
                Ok(content) => {
                    return if capture_output {
                        Ok(content)
                    } else {
                        println!("Read content: {}", content);
                        Ok(String::new())
                    }
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        Err(String::from("Permission denied"))
                    },
                    std::io::ErrorKind::NotFound => {
                        Err(String::from("File does not exist"))
                    },
                    _ => Err(String::from("Error reading file"))
                },
            }
        }

        _ => {
            if capture_output {
                Err(format!(
                    "Command '{}' cannot be used in variable assignment",
                    args[0]
                ))
            } else {
                Err(format!("Unknown command {}", args[0]))
            }
        }
    }
}
