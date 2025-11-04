pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    if args.len() < 5 {
        return Err("Usage: for <variable> in <start>..<end> do <command>".to_string());
    }

    let var_name = &args[1];
    if args[2] != "in" {
        return Err("Expected 'in' after variable name".to_string());
    }

    let range_str = &args[3];
    if !range_str.contains("..") {
        return Err("Expected range in format start..end".to_string());
    }

    let command_args_slice: &[String] = if let Some(do_idx) = args.iter().position(|a| a == "do") {
        &args[do_idx + 1..]
    } else if let Some(brace_idx) = args.iter().position(|a| a == "{") {
        &args[brace_idx..]
    } else {
        &args[4..]
    };

    let range_parts: Vec<&str> = range_str.split("..").collect();
    if range_parts.len() != 2 {
        return Err("Invalid range format. Use: start..end".to_string());
    }

    let start = range_parts[0]
        .parse::<i32>()
        .map_err(|_| "Start must be an integer".to_string())?;
    let end = range_parts[1]
        .parse::<i32>()
        .map_err(|_| "End must be an integer".to_string())?;

    let is_block = !command_args_slice.is_empty() && command_args_slice[0] == "{";
    let commands = if is_block {
        let block_content = if command_args_slice.len() > 1 {
            command_args_slice[1..command_args_slice.len() - 1].join(" ")
        } else {
            String::new()
        };
        crate::key_forge::key_forge::parse_block_commands(block_content.trim())
    } else {
        vec![command_args_slice.join(" ")]
    };

    for i in start..end {
        let mut store = crate::key_forge::key_forge::get_variable_store().lock().unwrap();
        store.add_data_to_int(var_name.to_string(), i);
        drop(store);

        for cmd in &commands {
            if crate::key_forge::key_forge::should_break() {
                crate::key_forge::key_forge::reset_loop_flags();
                return Ok(String::new());
            }
            if crate::key_forge::key_forge::should_continue() {
                break;
            }

            let cmd_args = crate::key_forge::input_mode::tokenize_input(cmd);
            crate::key_forge::execute_command::execute_command(&cmd_args, false)?;
        }

        if crate::key_forge::key_forge::should_continue() {
            crate::key_forge::key_forge::set_continue_flag(false);
        }
    }

    Ok(String::new())
}
