use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::key_forge::variables::{ParsedValue, Variables};

lazy_static! {
    static ref VARIABLE_STORE: Mutex<Variables> = Mutex::new(Variables::new());
}

pub fn get_variable_store() -> &'static Mutex<Variables> {
    &*VARIABLE_STORE
}

pub fn store_parsed_value(name: String, value: ParsedValue, _source: Option<&str>) -> Result<(), String> {
    let mut store = get_variable_store().lock().map_err(|e| format!("Mutex poisoned: {}", e))?;

    match value {
        ParsedValue::Int(iv) => store.add_data_to_int(name, iv),
        ParsedValue::Float(fv) => store.add_data_to_float(name, fv),
        ParsedValue::String(sv) => store.add_data_to_string(name, sv),
        ParsedValue::Array(arr) => store.add_data_to_array(name, arr),
        ParsedValue::Dictionary(dict) => store.add_data_to_dict(name, dict),
    }

    Ok(())
}

// Update save_state_to_file and load_state_from_file to handle arrays and dictionaries
pub fn save_state_to_file(filename: &str, store: &Variables) -> Result<(), String> {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(filename)
        .map_err(|e| format!("Failed to create file '{}': {}", filename, e))?;
    
    // Save integer variables
    for (name, value) in &store.int_variables {
        writeln!(file, "int:{}:{}", name, value)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    // Save float variables
    for (name, value) in &store.float_variables {
        writeln!(file, "float:{}:{}", name, value)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    // Save string variables (escape newlines and colons)
    for (name, value) in &store.string_variables {
        let escaped_value = value.replace("\\", "\\\\").replace(":", "\\:").replace("\n", "\\n");
        writeln!(file, "string:{}:{}", name, escaped_value)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    // Save array variables (using JSON serialization)
    for (name, value) in &store.array_variables {
        let json_value = serde_json::to_string(value)
            .map_err(|e| format!("Failed to serialize array '{}': {}", name, e))?;
        let escaped_json = json_value.replace("\\", "\\\\").replace(":", "\\:").replace("\n", "\\n");
        writeln!(file, "array:{}:{}", name, escaped_json)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    // Save dictionary variables (using JSON serialization)
    for (name, value) in &store.dict_variables {
        let json_value = serde_json::to_string(value)
            .map_err(|e| format!("Failed to serialize dict '{}': {}", name, e))?;
        let escaped_json = json_value.replace("\\", "\\\\").replace(":", "\\:").replace("\n", "\\n");
        writeln!(file, "dict:{}:{}", name, escaped_json)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    Ok(())
}

pub fn load_state_from_file(filename: &str, store: &mut Variables) -> Result<(), String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    let file = File::open(filename)
        .map_err(|e| format!("Failed to open file '{}': {}", filename, e))?;
    
    let reader = BufReader::new(file);
    
    // Clear existing variables before loading
    store.int_variables.clear();
    store.float_variables.clear();
    store.string_variables.clear();
    store.array_variables.clear();
    store.dict_variables.clear();
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| format!("Failed to read line {}: {}", line_num + 1, e))?;
        let line = line.trim();
        
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        
        let parts: Vec<&str> = line.splitn(3, ':').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid format at line {}: expected 'type:name:value'", line_num + 1));
        }
        
        let var_type = parts[0];
        let name = parts[1];
        let value = parts[2];
        
        match var_type {
            "int" => {
                let int_value = value.parse::<i32>()
                    .map_err(|e| format!("Invalid integer value at line {}: {}", line_num + 1, e))?;
                store.add_data_to_int(name.to_string(), int_value);
            }
            "float" => {
                let float_value = value.parse::<f64>()
                    .map_err(|e| format!("Invalid float value at line {}: {}", line_num + 1, e))?;
                store.add_data_to_float(name.to_string(), float_value);
            }
            "string" => {
                let unescaped_value = value.replace("\\n", "\n").replace("\\:", ":").replace("\\\\", "\\");
                store.add_data_to_string(name.to_string(), unescaped_value);
            }
            "array" => {
                let unescaped_value = value.replace("\\n", "\n").replace("\\:", ":").replace("\\\\", "\\");
                let array_value: Vec<ParsedValue> = serde_json::from_str(&unescaped_value)
                    .map_err(|e| format!("Invalid array value at line {}: {}", line_num + 1, e))?;
                store.add_data_to_array(name.to_string(), array_value);
            }
            "dict" => {
                let unescaped_value = value.replace("\\n", "\n").replace("\\:", ":").replace("\\\\", "\\");
                let dict_value: HashMap<String, ParsedValue> = serde_json::from_str(&unescaped_value)
                    .map_err(|e| format!("Invalid dict value at line {}: {}", line_num + 1, e))?;
                store.add_data_to_dict(name.to_string(), dict_value);
            }
            _ => return Err(format!("Unknown variable type '{}' at line {}", var_type, line_num + 1)),
        }
    }
    
    Ok(())
}