use rand::Rng;
use std::env;
use std::io;

mod utils;

mod key_forge {
    use super::*;

    fn get_random_char(mode: i32) -> char {
        let chars: &str;
        // big char mode
        if mode == 1 {
            chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        } else {
            chars = "abcdefghijklmnopqrstuvwxyz";
        }

        let mut rng = rand::thread_rng();
        let letters: Vec<char> = chars.chars().collect();
        let random_index = rng.gen_range(0..letters.len());

        letters[random_index]
    }

    fn get_random_num<T: rand::distributions::uniform::SampleUniform + std::cmp::PartialOrd>(
        min: T,
        max: T,
    ) -> T {
        rand::thread_rng().gen_range(min..=max)
    }

    pub fn interpret_arguments_from_command_line(args: &[String]) -> Result<(), String> {
        // Try parsing as integers first

        match args[0].as_str() {
            "get_random_num" => {
                if args.len() != 3 {
                    return Err("Usage: get_random_num <min> <max>".to_string());
                }

                // try parsing as int
                if let (Ok(min), Ok(max)) = (args[1].parse::<i32>(), args[2].parse::<i32>()) {
                    if min >= max {
                        return Err("min must be less than max".to_string());
                    }
                    println!("Random integer: {}", get_random_num(min, max));
                    return Ok(());
                }

                // Try parsing as floats
                if let (Ok(min), Ok(max)) = (args[1].parse::<f64>(), args[2].parse::<f64>()) {
                    if min >= max {
                        return Err("min must be less than max".to_string());
                    }
                    println!("Random float: {:.4}", get_random_num(min, max));
                    return Ok(());
                }
            }
            "get_random_char" => {
            if args.len() == 2 {
                match args[1].parse::<i32>() {
                    Ok(mode) => {
                        let c: char = get_random_char(mode);
                        println!("{}", c);
                    }
                    Err(_) => {
                        println!("invalid: {}. use number.", args[1]);
                        let c: char = get_random_char(0);
                        println!("{}", c);
                    }
                };
            } else {
                get_random_char(0); // default mode
            }
            }

            // no command
            _ => {}
        }

        Err("Arguments must be numbers (integers or floats)".to_string())
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
                        "quit" | "exit" => {
                            break;
                        }
                        "help" => {
                            println!("get_random_num : use for get random num with diapason");
                            println!("Examples:");
                            println!(
                                " get_random_num 1 100    - generates random integer between 1-100"
                            );
                            println!(
                                " get_random_num 0.5 5.5  - generates random float between 0.5-5.5"
                            );
                            continue;
                        }
                        "" => continue, // empty input
                        _ => {
                            let args: Vec<String> = tokenize_input(input);
                            match interpret_arguments_from_command_line(&args) {
                                Ok(()) => {}
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}. Please try again.", e);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        key_forge::cli_mode();
    } else {
        if let Err(e) = key_forge::interpret_arguments_from_command_line(&args) {
            eprintln!("Error: {}", e);
            println!("\nUsage examples:");
            println!(
                "  {} 1 100",
                env::args().next().unwrap_or("program".to_string())
            );
            println!(
                "  {} 0.5 5.5",
                env::args().next().unwrap_or("program".to_string())
            );
            std::process::exit(1);
        }
    }
}