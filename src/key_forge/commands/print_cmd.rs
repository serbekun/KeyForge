use crate::key_forge::key_forge::substitute_variables_in_string;
use crate::key_forge::input_mode;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: print <name or literal>".to_string());
    }

    let raw_value = args[1..].join(" ");

    if raw_value.starts_with("$(") && raw_value.ends_with(')') {
        let command_content = &raw_value[2..raw_value.len() - 1];
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);

        match crate::key_forge::execute_command::execute_command(&command_args, capture_output) {
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

    let substituted_string = substitute_variables_in_string(&raw_value);

    if capture_output {
        Ok(substituted_string)
    } else {
        println!("{}", substituted_string);
        Ok(String::new())
    }
}
