use crate::key_forge::key_forge::{get_variable_store, is_valid_identifier};
use crate::key_forge::input_mode;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: remove_string_char <variable_name> <index>".to_string());
    }

    let name = &args[1];
    let index_arg = &args[2];
    let mut store = get_variable_store().lock().unwrap();

    let index_value = if index_arg.starts_with("$(") && index_arg.ends_with(')') {
        let command_content = &index_arg[2..index_arg.len() - 1];
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&command_args, true) {
            Ok(output) => output
                .trim()
                .parse::<i32>()
                .map_err(|_| "Command output must be a valid integer".to_string())?,
            Err(e) => return Err(format!("Error executing index command: {}", e)),
        }
    } else {
        if index_arg.starts_with('$') && is_valid_identifier(&index_arg[1..]) {
            let var_name = &index_arg[1..];
            if let Ok(int_val) = store.get_int_data(var_name) {
                int_val
            } else {
                return Err(format!("Variable {} not found or not an integer", var_name));
            }
        } else {
            index_arg
                .parse::<i32>()
                .map_err(|_| "Index must be a valid integer".to_string())?
        }
    };

    if index_value < 0 {
        return Err("Index cannot be negative".to_string());
    }

    match store.remove_string_char(name, index_value as usize) {
        Ok(()) => {
            if capture_output {
                store
                    .get_string_data(name)
                    .map_err(|e| format!("Error getting updated string: {}", e))
            } else {
                Ok(String::new())
            }
        }
        Err(e) => Err(e),
    }
}
