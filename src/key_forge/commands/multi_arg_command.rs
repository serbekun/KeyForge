pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: multi_arg_command <arg1> <arg2> ...".to_string());
    }

    let processed_args = &args[1..].join(" ");

    for (i, arg) in processed_args.chars().enumerate() {
        println!("Arg {}: {}", i + 1, arg);
    }

    Ok(String::new())
}
