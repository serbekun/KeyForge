use crate::key_forge::key_forge::get_variable_store;

pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    let mut store = get_variable_store().lock().unwrap();
    let k = &args[1];

    if store.int_variables.contains_key(k) {
        store.remove_int_data(k);
        return Ok(String::new());
    }

    if store.float_variables.contains_key(k) {
        store.remove_float_data(k);
        return Ok(String::new());
    }

    if store.string_variables.contains_key(k) {
        store.remove_string_data(k);
        return Ok(String::new());
    }

    Err(format!("Variable {} not found", k))
}
