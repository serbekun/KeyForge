use crate::key_forge::input_mode;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 4 {
        return Err("Usage: if <condition> then <command> [else <command>]".to_string());
    }

    let then_index: usize = args
        .iter()
        .position(|arg| arg == "then")
        .ok_or("Expected 'then' after condition".to_string())?;

    let condition_parts = &args[1..then_index];
    let condition = condition_parts.join(" ");

    let condition_result: bool = crate::key_forge::key_forge::evaluate_condition(&condition)?;

    if condition_result {
        let then_command_start: usize = then_index + 1;
        let else_index = args.iter().position(|arg| arg == "else");

        let then_args = if let Some(else_idx) = else_index {
            &args[then_command_start..else_idx]
        } else {
            &args[then_command_start..]
        };

        if !then_args.is_empty() {
            let is_block = then_args[0] == "{";
            if is_block {
                let block_content = if then_args.len() > 1 {
                    then_args[1..then_args.len() - 1].join(" ")
                } else {
                    String::new()
                };
                let commands = crate::key_forge::key_forge::parse_block_commands(block_content.trim());
                for cmd in &commands {
                    let cmd_args = input_mode::tokenize_input(cmd);
                    crate::key_forge::execute_command::execute_command(&cmd_args, capture_output)?;
                }
            } else {
                crate::key_forge::execute_command::execute_command(then_args, capture_output)?;
            }
        }
    } else {
        if let Some(else_index) = args.iter().position(|arg| arg == "else") {
            let else_args = &args[else_index + 1..];
            if !else_args.is_empty() {
                let is_block = else_args[0] == "{";
                if is_block {
                    let block_content = if else_args.len() > 1 {
                        else_args[1..else_args.len() - 1].join(" ")
                    } else {
                        String::new()
                    };
                    let commands = crate::key_forge::key_forge::parse_block_commands(block_content.trim());
                    for cmd in &commands {
                        let cmd_args = input_mode::tokenize_input(cmd);
                        crate::key_forge::execute_command::execute_command(&cmd_args, capture_output)?;
                    }
                } else {
                    crate::key_forge::execute_command::execute_command(else_args, capture_output)?;
                }
            }
        }
    }

    Ok(String::new())
}
