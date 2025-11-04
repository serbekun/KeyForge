pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 4 {
        return Err("Usage: while <condition> do <command>".to_string());
    }

    let do_index: usize = args
        .iter()
        .position(|arg| arg == "do")
        .ok_or("Expected 'do' after condition".to_string())?;

    let condition_parts = &args[1..do_index];
    let command_args = &args[do_index + 1..];

    if command_args.is_empty() {
        return Err("No command specified after 'do'".to_string());
    }

    let is_block = !command_args.is_empty() && command_args[0] == "{";
    let commands = if is_block {
        let block_content = if command_args.len() > 1 {
            command_args[1..command_args.len() - 1].join(" ")
        } else {
            String::new()
        };
        crate::key_forge::key_forge::parse_block_commands(&block_content)
    } else {
        vec![command_args.join(" ")]
    };

    let condition = condition_parts.join(" ");

    loop {
        if crate::key_forge::key_forge::should_break() {
            crate::key_forge::key_forge::reset_loop_flags();
            break;
        }

        let condition_result = crate::key_forge::key_forge::evaluate_condition(&condition)?;
        if !condition_result {
            break;
        }

        crate::key_forge::key_forge::set_continue_flag(false);

        for cmd in &commands {
            if crate::key_forge::key_forge::should_break() {
                break;
            }
            if crate::key_forge::key_forge::should_continue() {
                break;
            }

            let cmd_args = crate::key_forge::input_mode::tokenize_input(cmd);
            crate::key_forge::execute_command::execute_command(&cmd_args, capture_output)?;
        }

        if crate::key_forge::key_forge::should_break() {
            crate::key_forge::key_forge::reset_loop_flags();
            break;
        }

        if crate::key_forge::key_forge::should_continue() {
            crate::key_forge::key_forge::set_continue_flag(false);
        }
    }

    Ok(String::new())
}
