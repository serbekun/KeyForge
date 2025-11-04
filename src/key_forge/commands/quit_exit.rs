use colored::Colorize;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if capture_output {
        return Err("quit/exit cannot be used in variable assignment".to_string());
    }

    if args.len() >= 2 {
        match args[1].parse::<i32>() {
            Ok(exit_code) => {
                println!(
                    "{}",
                    format!("Program exit with code {}", exit_code).green().bold()
                );
                std::process::exit(exit_code);
            }
            Err(_) => {
                println!("{}", "Program exit with code 0".green().bold());
                std::process::exit(0);
            }
        }
    }

    println!("{}", "Program exit with code 0".green().bold());
    std::process::exit(0);
}
