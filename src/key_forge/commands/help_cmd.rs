use crate::key_forge::help;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if capture_output {
        return Err("help cannot be used in variable assignment".to_string());
    }

    if args.len() >= 2 {
        help::show_command_help(&args[1]);
    } else {
        help::show_all_help();
    }

    Ok(String::new())
}
