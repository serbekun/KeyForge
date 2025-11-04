use crate::key_forge::input_mode;
use crate::key_forge::key_forge::get_variable_store;
use crate::key_forge::key_forge::utils;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err(String::from("Usage: read_file <filename>"));
    }

    let filename = &args[1];

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

    let result = utils::read_from_file(&filename);

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
            std::io::ErrorKind::PermissionDenied => Err(String::from("Permission denied")),
            std::io::ErrorKind::NotFound => Err(String::from("File does not exist")),
            _ => Err(String::from("Error reading file")),
        },
    }
}
