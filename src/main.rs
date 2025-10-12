use rand::Rng;
use std::env;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use colored::*;

mod utils;

mod key_forge {
    use super::*;

    fn help() {
        println!("{}" ,"get_random_num : use for get random num with diapason".blue());
        println!("Examples:");
        println!(" get_random_num 1 100    - generates random integer between 1-100");
        println!(" get_random_num 0.5 5.5  - generates random float between 0.5-5.5");
        println!("get_random_char : use for get random char from alphabet");
        println!("{}" ,"Examples:".blue());
        println!(" get_random_char   - return random char example 'a'");
        println!(" get_random_char 1 - return random big char example 'B'")
    }

    fn get_random_char(mode: i32) -> Result<char, String> {
        let chars: &str;
        
        if mode != 1 && mode != 0 {
            return Err(format!("Error: Unknown mode '{}'. Mode must be 0 or 1.", mode));
        }
        
        if mode == 1 {
            chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        } else {
            chars = "abcdefghijklmnopqrstuvwxyz";
        }

        let mut rng = rand::thread_rng();
        let letters: Vec<char> = chars.chars().collect();
        let random_index = rng.gen_range(0..letters.len());

        Ok(letters[random_index])
    }

    fn get_random_num<T: rand::distributions::uniform::SampleUniform + std::cmp::PartialOrd>(
        min: T,
        max: T,
    ) -> T {
        rand::thread_rng().gen_range(min..=max)
    }

    pub fn interpret_arguments_from_command_line(args: &[String]) -> Result<(), String> {
        if args.is_empty() {
            return Ok(());
        }

        match args[0].as_str() {
            "quit" | "exit" => {
                if args.len() >= 2 {
                    match args[1].parse::<i32>() {
                        Ok(exit_code) => {
                            println!("{}", format!("Program exit with code {}", exit_code).green().bold());
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
            
            "help" => {
                help();
                Ok(())
            }

            "get_random_num" => {
                if args.len() != 3 {
                    return Err(format!("Usage: get_random_num <min> <max>"));
                }

                // Try parsing as int
                if let (Ok(min), Ok(max)) = (args[1].parse::<i32>(), args[2].parse::<i32>()) {
                    if min >= max {
                        return Err("min must be less than max".to_string());
                    }
                    println!("Random integer: {}", get_random_num(min, max));
                    return Ok(())
                }

                // Try parsing as floats
                if let (Ok(min), Ok(max)) = (args[1].parse::<f64>(), args[2].parse::<f64>()) {
                    if min >= max {
                        return Err("min must be less than max".to_string());
                    }
                    println!("Random float: {:.4}", get_random_num(min, max));
                    return Ok(());
                }
                Err("Arguments must be numbers (integers or floats)".to_string())
            }

            "get_random_char" => {
                if args.len() == 2 {
                    match args[1].parse::<i32>() {
                        Ok(mode) => {
                            match get_random_char(mode) {
                                Ok(c) => println!("{}", c),
                                Err(e) => return Err(e),
                            }
                        }
                        Err(_) => {
                            println!("Invalid mode: '{}'. Using default mode 0.", args[1]);
                            match get_random_char(0) {
                                Ok(c) => println!("{}", c),
                                Err(e) => return Err(e),
                            }
                        }
                    };
                } else {
                    match get_random_char(0) {
                        Ok(c) => println!("{}", c),
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }

            // no command
            _ => {
                Err(format!("Unknown command {}", args[0]))
            }
        }
    }

    fn tokenize_input(input: &str) -> Vec<String> {
        input.split_whitespace().map(|s| s.to_string()).collect()
    }

    pub fn cli_mode() {
        let mut input = String::new();

        println!("key forge CLI");

        loop {
            print!("> ");
            utils::flush_stdout();
            input.clear();

            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();

                    match input.to_lowercase().as_str() {
                        "" => continue, // empty input
                        _ => {
                            let args: Vec<String> = tokenize_input(input);
                            match interpret_arguments_from_command_line(&args) {
                                Ok(()) => {}
                                Err(e) => eprintln!("{}" ,format!("Error: {}", e).red().bold()),
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", format!("Error reading input: {}. Please try again.", e).red().bold());
                }
            }
        }
    }

    pub fn file_mode(filename: &String) {
        let file = File::open(filename).expect("Error open file");
        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let command = match line {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("{}", format!("Error reading line: {}", e).red().bold());
                    eprintln!("{}", "Stop interpret program".red().bold());
                    break;
                }
            };

            let args: Vec<String> = tokenize_input(&command);

            if let Err(e) = key_forge::interpret_arguments_from_command_line(&args) {
                println!("{}", format!("in line {}: {}", line_num, e).red().bold());
                println!("{}" ,"Stop interpret program".red().bold());
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        key_forge::cli_mode();
    } 
    if args[0] == "file" {
        key_forge::file_mode(&args[1]);
    }
    else {
        if let Err(e) = key_forge::interpret_arguments_from_command_line(&args) {
            println!("{}" ,format!("{}", e).red().bold());
            std::process::exit(1);
        }
    }
}