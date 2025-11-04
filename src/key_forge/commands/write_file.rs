use crate::key_forge::input_mode;
use crate::key_forge::key_forge::get_variable_store;
use crate::key_forge::key_forge::utils;

pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    if args.len() < 4 {
        return Err(String::from("Usage: write_file <filename> <content> <append>"));
    }

    let filename = &args[1];
    let content = &args[2];
    let append = &args[3];

    let filename = if filename.starts_with("$(") && filename.ends_with(')') {
        let command_content = &filename[2..filename.len() - 1];
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&command_args, true) {
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
        let command_content = &content[2..content.len() - 1];
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&command_args, true) {
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

    let append = if append.starts_with("$(") && append.ends_with(')') {
        let command_content = &append[2..append.len() - 1];
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&command_args, true) {
            Ok(output) => output,
            Err(e) => return Err(e),
        }
    } else if append.starts_with('$') {
        let var_name = &append[1..];
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

    let result = utils::write_to_file_with_mode(&filename, &content, should_append);

    match result {
        Ok(()) => Ok(String::new()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::PermissionDenied => Err(String::from("Permission denied")),
            std::io::ErrorKind::NotFound => Err(String::from("Directory does not exist")),
            std::io::ErrorKind::AlreadyExists => Err(String::from("File exist but blocked")),
            _ => Err(String::from("Error write file")),
        },
    }
}
