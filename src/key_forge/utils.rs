use std::io::Write;
use std::fs::OpenOptions;
use rand::Rng;
use crate::key_forge::store::get_variable_store;
use crate::key_forge::variables::{ParsedValue, value_to_string};
use crate::key_forge::parser::parse_value;
use crate::key_forge::input_mode;

pub fn resolve_to_string(value: &str) -> Result<String, String> {
    let key = if value.starts_with('$') { &value[1..] } else { value };

    let store = get_variable_store().lock().unwrap();

    if let Ok(int_val) = store.get_int_data(key) {
        Ok(int_val.to_string())
    } else if let Ok(float_val) = store.get_float_data(key) {
        Ok(float_val.to_string())
    } else if let Ok(string_val) = store.get_string_data(key) {
        Ok(string_val)
    } else if let Ok(array_val) = store.get_array_data(key) {
        // Convert array to string representation
        let elements: Vec<String> = array_val.iter().map(|v| value_to_string(v)).collect();
        Ok(format!("[{}]", elements.join(", ")))
    } else if let Ok(dict_val) = store.get_dict_data(key) {
        // Convert dictionary to string representation
        let pairs: Vec<String> = dict_val.iter()
            .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
            .collect();
        Ok(format!("{{{}}}", pairs.join(", ")))
    } else {
        // Variable doesn't exist - try parsing as literal
        let parsed_value = parse_value(value);
        match parsed_value {
            ParsedValue::Int(i) => Ok(i.to_string()),
            ParsedValue::Float(f) => Ok(f.to_string()),
            ParsedValue::String(s) => Ok(s),
            ParsedValue::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| value_to_string(v)).collect();
                Ok(format!("[{}]", elements.join(", ")))
            }
            ParsedValue::Dictionary(dict) => {
                let pairs: Vec<String> = dict.iter()
                    .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                    .collect();
                Ok(format!("{{{}}}", pairs.join(", ")))
            }
        }
    }
}

pub fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        if !first.is_alphabetic() && first != '_' {
            return false;
        }
    } else {
        return false;
    }

    chars.all(|c| c.is_alphanumeric() || c == '_')
}

pub fn get_random_char(mode: i32) -> Result<char, String> {
    let mut rng = rand::thread_rng();
    let base = if mode == 1 { 'A' as u8 } else { 'a' as u8 };
    let offset = rng.gen_range(0..26);
    Ok((base + offset) as char)
}

// Generic wrapper used by execute_command which expects `get_random_num` generic
pub fn get_random_num<T>(min: T, max: T) -> T
where
    T: PartialOrd + Copy + rand::distributions::uniform::SampleUniform,
{
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn substitute_variables_in_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '$' {
            // Start reading variable name
            let mut var_name = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    var_name.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            
            if !var_name.is_empty() {
                // Try to get variable value
                let store = get_variable_store().lock().unwrap();
                if let Ok(int_val) = store.get_int_data(&var_name) {
                    result.push_str(&int_val.to_string());
                } else if let Ok(float_val) = store.get_float_data(&var_name) {
                    result.push_str(&float_val.to_string());
                } else if let Ok(string_val) = store.get_string_data(&var_name) {
                    result.push_str(&string_val);
                } else {
                    // Variable not found, leave as is
                    result.push('$');
                    result.push_str(&var_name);
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

pub fn write_to_file_with_mode(filename: &str, content: &str, append: bool) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(append)      // if true - append to end
        .truncate(!append)   // if false - cutting file (rewrite)
        .create(true)        // create if not exist
        .open(filename)?;
    
    writeln!(&mut file, "{}", content)?;
    Ok(())
}

pub fn read_from_file(filename: &str) -> std::io::Result<String> {
    std::fs::read_to_string(filename)
}

pub fn wrap_string(s: &str, wrapper: char) -> String {
    format!("{}{}{}", wrapper, s, wrapper)
}

pub fn resolve_filename(filename_raw: &str) -> Result<String, String> {
    // Handle command substitution: $(command)
    if filename_raw.starts_with("$(") && filename_raw.ends_with(')') {
        let command_content = &filename_raw[2..filename_raw.len()-1];
        let command_args: Vec<String> = input_mode::tokenize_input(command_content);
        
        match crate::key_forge::execute_command::execute_command(&command_args, true) {
            Ok(result) => Ok(result.trim().to_string()),
            Err(e) => Err(format!("Error executing command: {}", e)),
        }
} 
    // Handle variable reference: $variable
    else if filename_raw.starts_with('$') && is_valid_identifier(&filename_raw[1..]) {
        let var_name = &filename_raw[1..];
        let store = get_variable_store().lock().unwrap();
        
        if let Ok(string_val) = store.get_string_data(var_name) {
            Ok(string_val)
        } else {
            Err(format!("String variable {} not found", var_name))
        }
    }
    // Direct filename (remove quotes if present)
    else {
        let filename = filename_raw.trim();
        let filename = filename.strip_prefix('"').unwrap_or(filename);
        let filename = filename.strip_suffix('"').unwrap_or(filename);
        let filename = filename.strip_prefix('\'').unwrap_or(filename);
        let filename = filename.strip_suffix('\'').unwrap_or(filename);
        
        Ok(filename.to_string())
    }
}