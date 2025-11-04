use crate::key_forge::key_forge::get_variable_store;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: base64_encode <encode string variable name>".to_string());
    }

    let string = &args[1];

    let string = if string.starts_with("$(") && string.ends_with(')') {
        let command_content = &string[2..string.len() - 1];
        let command_args: Vec<String> = crate::key_forge::input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&command_args, true) {
            Ok(output) => output,
            Err(e) => return Err(e),
        }
    } else if string.starts_with('$') {
        let var_name = &string[1..];
        let store = get_variable_store().lock().unwrap();
        match store.get_string_data(var_name) {
            Ok(value) => value,
            Err(e) => return Err(format!("Undefined variable: {}", e)),
        }
    } else {
        string.to_string()
    };

    let encode_string = crate::key_forge::key_forge::base64::encode_base64(&string);

    if !capture_output {
        println!("{}", encode_string);
        return Ok(String::new());
    } else {
        return Ok(encode_string);
    }
}
