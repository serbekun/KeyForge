use crate::key_forge::key_forge::{resolve_to_string, utils};
use crate::key_forge::input_mode;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: num_to_string <source>".to_string());
    }

    let raw_value = args[1..].join(" ").trim().to_string();

    let string_val = if raw_value.starts_with("$(") && raw_value.ends_with(')') {
        let command_content = &raw_value[2..raw_value.len() - 1];
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&command_args, true) {
            Ok(result) => result,
            Err(e) => return Err(format!("Error executing command: {}", e)),
        }
    } else {
        resolve_to_string(&raw_value)?
    };

    let string_val = utils::wrap_string(&string_val, '"');

    if capture_output {
        Ok(string_val)
    } else {
        println!("num_to_string: {}", string_val);
        Ok(String::new())
    }
}
