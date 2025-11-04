use crate::key_forge::expression;
use crate::key_forge::key_forge::get_variable_store;

pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: push <array_name> <value>".to_string());
    }

    let array_name = &args[1];
    let value_str = &args[2..].join(" ");

    let parsed_value = expression::evaluate_expression(value_str)?;

    let mut store = get_variable_store().lock().unwrap();

    if let Ok(mut array) = store.get_array_data(array_name) {
        array.push(parsed_value);
        store.add_data_to_array(array_name.to_string(), array);
        Ok(String::new())
    } else {
        let new_array = vec![parsed_value];
        store.add_data_to_array(array_name.to_string(), new_array);
        Ok(String::new())
    }
}
