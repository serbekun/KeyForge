use crate::key_forge::key_forge::{get_variable_store, is_valid_identifier};
use crate::key_forge::input_mode;

pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: push_to_string_back <variable_name> <value>".to_string());
    }

    let var_name = &args[1];
    let raw_value = args[2..].join(" ");

    let value_to_push = if raw_value.starts_with("$(") && raw_value.ends_with(')') {
        let command_content = &raw_value[2..raw_value.len() - 1];
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&command_args, true) {
            Ok(output) => output,
            Err(e) => return Err(format!("Error executing inner command: {}", e)),
        }
    } else {
        if is_valid_identifier(&raw_value) {
            let store = get_variable_store().lock().unwrap();

            if let Ok(int_val) = store.get_int_data(&raw_value) {
                int_val.to_string()
            } else if let Ok(float_val) = store.get_float_data(&raw_value) {
                float_val.to_string()
            } else if let Ok(string_val) = store.get_string_data(&raw_value) {
                string_val
            } else {
                raw_value
            }
        } else {
            raw_value
        }
    };

    let mut store = get_variable_store().lock().unwrap();

    if let Ok(current_value) = store.get_string_data(var_name) {
        let new_value = current_value + &value_to_push;
        store.add_data_to_string(var_name.to_string(), new_value);
        Ok(String::new())
    } else {
        store.add_data_to_string(var_name.to_string(), value_to_push);
        Ok(String::new())
    }
}
