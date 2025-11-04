pub fn run(_args: &[String], capture_output: bool) -> Result<String, String> {
    if !capture_output {
        crate::key_forge::key_forge::set_continue_flag(true);
        Ok(String::new())
    } else {
        Err("continue cannot be used in variable assignment".to_string())
    }
}
