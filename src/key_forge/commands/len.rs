use crate::key_forge::key_forge::get_variable_store;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: len <variable_name>".to_string());
    }

    let var_name = &args[1];
    let store = get_variable_store().lock().unwrap();

    let length = if let Ok(array) = store.get_array_data(var_name) {
        array.len()
    } else if let Ok(dict) = store.get_dict_data(var_name) {
        dict.len()
    } else if let Ok(string) = store.get_string_data(var_name) {
        string.chars().count()
    } else {
        return Err(format!("Variable '{}' not found or not a collection/string", var_name));
    };

    if capture_output {
        Ok(length.to_string())
    } else {
        println!("{}", length);
        Ok(String::new())
    }
}
