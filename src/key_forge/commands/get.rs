use crate::key_forge::key_forge::get_variable_store;
use crate::key_forge::key_forge::value_to_string;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: get <collection_name> <key/index>".to_string());
    }

    let collection_name = &args[1];
    let key_str = &args[2];
    let store = get_variable_store().lock().unwrap();

    if let Ok(array) = store.get_array_data(collection_name) {
        let index: usize = key_str
            .parse()
            .map_err(|_| "Array index must be a non-negative integer".to_string())?;

        if index < array.len() {
            let result = value_to_string(&array[index]);
            if capture_output {
                Ok(result)
            } else {
                println!("{}", result);
                Ok(String::new())
            }
        } else {
            Err(format!("Index {} out of bounds for array '{}'", index, collection_name))
        }
    } else if let Ok(dict) = store.get_dict_data(collection_name) {
        if let Some(value) = dict.get(key_str) {
            let result = value_to_string(value);
            if capture_output {
                Ok(result)
            } else {
                println!("{}", result);
                Ok(String::new())
            }
        } else {
            Err(format!("Key '{}' not found in dictionary '{}'", key_str, collection_name))
        }
    } else {
        Err(format!("Collection '{}' not found", collection_name))
    }
}
