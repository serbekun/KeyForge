use std::collections::HashMap;
use crate::key_forge::execute_command::execute_command;
use crate::key_forge::input_mode::tokenize_input;
use crate::key_forge::parser::parse_value;
use crate::key_forge::store::get_variable_store;
use crate::key_forge::variables::ParsedValue;

#[allow(dead_code)]
pub fn set_simple_variable(args: &[String]) -> Result<String, String> {
    let name = args[1].clone();
    let raw_value = args[2..].join(" ");

    // Use our new expression evaluator that handles variables, commands and arrays
    match crate::key_forge::expression::evaluate_expression(&raw_value) {
        Ok(parsed_value) => {
            crate::key_forge::key_forge::store_parsed_value(name, parsed_value, None)?;
            Ok(String::new())
        }
        Err(e) => Err(format!("Error evaluating expression: {}", e)),
    }
}

pub fn set_collection_element(args: &[String]) -> Result<String, String> {
    let collection_name = &args[1];
    let key_str = &args[2];
    let value_str = &args[3..].join(" ");

    let parsed_value = if value_str.starts_with("$(") && value_str.ends_with(')') {
        let command_content = &value_str[2..value_str.len() - 1];
        let command_args: Vec<String> = tokenize_input(command_content);

        match execute_command(&command_args, true) {
            Ok(output) => parse_value(&output),
            Err(e) => return Err(format!("Error executing inner command: {}", e)),
        }
    } else {
        parse_value(value_str)
    };

    let mut store = get_variable_store().lock().unwrap();

    // Try as array first
    if let Ok(mut array) = store.get_array_data(collection_name) {
        let index: usize = key_str
            .parse()
            .map_err(|_| "Array index must be a non-negative integer".to_string())?;

        if index < array.len() {
            array[index] = parsed_value;
            store.add_data_to_array(collection_name.to_string(), array);
            Ok(String::new())
        } else {
            Err(format!(
                "Index {} out of bounds for array '{}'",
                index, collection_name
            ))
        }
    }
    // Try as dictionary
    else if let Ok(mut dict) = store.get_dict_data(collection_name) {
        dict.insert(key_str.to_string(), parsed_value);
        store.add_data_to_dict(collection_name.to_string(), dict);
        Ok(String::new())
    } else {
        // Collection doesn't exist - create a new one
        // Try to parse key as array index first
        if let Ok(index) = key_str.parse::<usize>() {
            // Create new array with the given value at the specified index
            // If index is larger than 0, fill previous positions with default values
            let mut new_array = Vec::new();
            if index > 0 {
                // Fill with empty values up to the index
                for _ in 0..index {
                    new_array.push(ParsedValue::String(String::new()));
                }
            }
            new_array.push(parsed_value);
            store.add_data_to_array(collection_name.to_string(), new_array);
            Ok(String::new())
        } else {
            // Create new dictionary with the key-value pair
            let mut new_dict = HashMap::new();
            new_dict.insert(key_str.to_string(), parsed_value);
            store.add_data_to_dict(collection_name.to_string(), new_dict);
            Ok(String::new())
        }
    }
}