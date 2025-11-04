use crate::key_forge::key_forge::{get_variable_store, value_to_string};

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: values <dict_name>".to_string());
    }

    let dict_name = &args[1];
    let store = get_variable_store().lock().unwrap();

    if let Ok(dict) = store.get_dict_data(dict_name) {
        let values: Vec<String> = dict.values().map(value_to_string).collect();
        let result = format!("[{}]", values.join(", "));

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
