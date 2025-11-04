use crate::key_forge::input_mode;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;

pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: to_file <filename> <command...>".to_string());
    }

    let filename = &args[1];
    let command_args = &args[2..];

    // Handle command substitution
    let raw_command = command_args.join(" ");
    if raw_command.starts_with("$(") && raw_command.ends_with(')') {
        let command_content = &raw_command[2..raw_command.len() - 1];
        let inner_command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&inner_command_args, true) {
            Ok(output) => {
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
        match crate::key_forge::execute_command::execute_command(command_args, true) {
            Ok(output) => {
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
