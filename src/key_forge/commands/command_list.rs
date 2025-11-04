use crate::key_forge::help;

pub fn run(_args: &[String], capture_output: bool) -> Result<String, String> {
    if capture_output {
        return Err("command_list cannot be used in variable assignment".to_string());
    }
    help::show_command_list();
    Ok(String::new())
}
