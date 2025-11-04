use crate::key_forge::input_mode;

pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err(format!("Usage: execute_file <filename>"));
    }

    input_mode::file_mode(&args[1]);
    Ok(String::new())
}
