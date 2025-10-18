use super::key_forge::{get_variable_store, ParsedValue};

// Helper function for arithmetic operations
pub fn perform_arithmetic(operation: &str, var_name: &str, value: ParsedValue) -> Result<(), String> {
    let mut store = get_variable_store().lock().unwrap();
    
    match operation {
        "add" => {
            match value {
                ParsedValue::Int(iv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_int(var_name.to_string(), val + iv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val + iv as f64);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_string_data(var_name) {
                        store.add_data_to_string(var_name.to_string(), val + &iv.to_string());
                        return Ok(());
                    }
                }
                ParsedValue::Float(fv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val as f64 + fv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val + fv);
                        return Ok(());
                    }
                }
                ParsedValue::String(sv) => {
                    if let Ok(val) = store.get_string_data(var_name) {
                        store.add_data_to_string(var_name.to_string(), val + &sv);
                        return Ok(());
                    }
                }
            }
            Err(format!("Variable {} not found or incompatible type", var_name))
        }
        "sub" => {
            match value {
                ParsedValue::Int(iv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_int(var_name.to_string(), val - iv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val - iv as f64);
                        return Ok(());
                    }
                }
                ParsedValue::Float(fv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val as f64 - fv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val - fv);
                        return Ok(());
                    }
                }
                _ => return Err("Cannot subtract non-numeric value".to_string()),
            }
            Err(format!("Variable {} not found or not a number", var_name))
        }
        "mul" => {
            match value {
                ParsedValue::Int(iv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_int(var_name.to_string(), val * iv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val * iv as f64);
                        return Ok(());
                    }
                }
                ParsedValue::Float(fv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val as f64 * fv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val * fv);
                        return Ok(());
                    }
                }
                _ => return Err("Cannot multiply by non-numeric value".to_string()),
            }
            Err(format!("Variable {} not found or not a number", var_name))
        }
        "div" => {
            match value {
                ParsedValue::Int(iv) => {
                    if iv == 0 {
                        return Err("Division by zero".to_string());
                    }
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_int(var_name.to_string(), val / iv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val / iv as f64);
                        return Ok(());
                    }
                }
                ParsedValue::Float(fv) => {
                    if fv == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val as f64 / fv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val / fv);
                        return Ok(());
                    }
                }
                _ => return Err("Cannot divide by non-numeric value".to_string()),
            }
            Err(format!("Variable {} not found or not a number", var_name))
        }
        _ => Err(format!("Unknown operation: {}", operation)),
    }
}