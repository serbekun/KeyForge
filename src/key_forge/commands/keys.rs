use crate::key_forge::key_forge::get_variable_store;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: keys <dict_name>".to_string());
    }

    let dict_name = &args[1];
    let store = get_variable_store().lock().unwrap();

    if let Ok(dict) = store.get_dict_data(dict_name) {
        let keys: Vec<String> = dict.keys().cloned().collect();
        let result = format!("[{}]", keys.join(", "));

        if capture_output {
            Ok(result)
        } else {
            println!("{}", result);
            Ok(String::new())
        }
    } else {
        Err(format!("Dictionary '{}' not found", dict_name))
    }
}
