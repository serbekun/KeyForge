use crate::key_forge::xor;
use crate::key_forge::key_forge::get_variable_store;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: xor <data> <key> [--decode]".to_string());
    }
    
    let data = resolve_input(&args[1])?;
    let key = resolve_input(&args[2])?;
    
    if data.is_empty() || key.is_empty() {
        return Err("Data and key cannot be empty".to_string());
    }

    // Check if decode mode is requested (looking for --decode flag)
    let decode_mode = args.iter().any(|arg| arg == "--decode");

    let result = if decode_mode {
        // DECODE MODE: input is hex string, output is plain text
        xor::decode_hex(&data, &key)?
    } else {
        // ENCODE MODE: input is plain text, output is hex string
        xor::encode_to_hex(&data, &key)?
    };
    
    if capture_output {
        Ok(result)
    } else {
        if decode_mode {
            println!("Decoded text: {}", result);
        } else {
            println!("XOR encoded: {}", result);
        }
        Ok(result)
    }
}

/// Helper function to resolve input that could be a direct value, variable, or command substitution
fn resolve_input(input: &str) -> Result<String, String> {
    if input.starts_with("$(") && input.ends_with(')') {
        // Command substitution
        let command_content = &input[2..input.len() - 1];
        let command_args: Vec<String> = crate::key_forge::input_mode::tokenize_input(command_content);
        crate::key_forge::execute_command::execute_command(&command_args, true)
    } else if input.starts_with('$') {
        // Variable reference
        let var_name = &input[1..];
        let store = get_variable_store().lock().unwrap();
        store.get_string_data(var_name)
            .map_err(|e| format!("Undefined variable: {}", e))
    } else {
        // Direct value
        Ok(input.to_string())
    }
}