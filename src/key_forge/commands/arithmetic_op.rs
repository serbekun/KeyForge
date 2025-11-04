use crate::key_forge::key_forge::{get_variable_store, is_valid_identifier, ParsedValue};
use crate::key_forge::arithmetic::perform_arithmetic;
use crate::key_forge::input_mode;

pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    let command_args = &args[2..];
    let raw_command = command_args.join(" ");

    if raw_command.starts_with("$(") && raw_command.ends_with(')') {
        let command_content = &raw_command[2..raw_command.len() - 1];
        let inner_command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&inner_command_args, true) {
            Ok(output) => {
                let parsed_value = crate::key_forge::key_forge::parse_value(&output);
                perform_arithmetic(&args[0], &args[1], parsed_value)?;
                Ok(String::new())
            }
            Err(e) => Err(format!("Error executing inner command: {}", e)),
        }
    } else {
        let parsed_value = if is_valid_identifier(&raw_command) {
            let store = get_variable_store().lock().unwrap();
            if store.has_variable(&raw_command) {
                if let Ok(int_val) = store.get_int_data(&raw_command) {
                    ParsedValue::Int(int_val)
                } else if let Ok(float_val) = store.get_float_data(&raw_command) {
                    ParsedValue::Float(float_val)
                } else if let Ok(string_val) = store.get_string_data(&raw_command) {
                    ParsedValue::String(string_val)
                } else {
                    crate::key_forge::key_forge::parse_value(&raw_command)
                }
            } else {
                crate::key_forge::key_forge::parse_value(&raw_command)
            }
        } else {
            crate::key_forge::key_forge::parse_value(&raw_command)
        };

        perform_arithmetic(&args[0], &args[1], parsed_value)?;
        Ok(String::new())
    }
}
