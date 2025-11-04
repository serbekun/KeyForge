use crate::key_forge::key_forge::{get_variable_store, value_to_string};

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: pop <array_name>".to_string());
    }

    let array_name = &args[1];
    let mut store = get_variable_store().lock().unwrap();

    if let Ok(mut array) = store.get_array_data(array_name) {
        if let Some(popped_value) = array.pop() {
            store.add_data_to_array(array_name.to_string(), array);
            let result = value_to_string(&popped_value);
            if capture_output {
                Ok(result)
            } else {
                println!("{}", result);
                Ok(String::new())
            }
        } else {
            Err("Array is empty".to_string())
        }
    } else {
        Err(format!("Array '{}' not found", array_name))
    }
}
