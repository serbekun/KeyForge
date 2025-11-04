use clear_screen::clear;

pub fn run(_args: &[String], capture_output: bool) -> Result<String, String> {
    if capture_output {
        return Err("Command 'clear' cannot be used in variable assignment".to_string());
    }

    clear();
    Ok(String::new())
}
