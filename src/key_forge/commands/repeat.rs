use crate::key_forge::key_forge::{get_variable_store, is_valid_identifier};
use crate::key_forge::input_mode;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: repeat <count> <command...>".to_string());
    }

    // Parse count - can be variable, command substitution, or direct number
    let count_raw = &args[1];
    let count = if count_raw.starts_with("$(") && count_raw.ends_with(')') {
        // Handle command substitution for count
        let command_content = &count_raw[2..count_raw.len() - 1];
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&command_args, true) {
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
            return Err(format!("Variable '{}' not found or not an integer", var_name));
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
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);

        for _ in 0..count {
            match crate::key_forge::execute_command::execute_command(&command_args, true) {
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
        let command_args: Vec<String> = input_mode::tokenize_input(&raw_command);

        for _ in 0..count {
            match crate::key_forge::execute_command::execute_command(&command_args, true) {
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
