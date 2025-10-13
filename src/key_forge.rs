use rand::Rng;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use colored::*;
use std::collections::HashMap;
use std::sync::{Mutex};
use std::sync::OnceLock;

use crate::utils;

use super::*;

pub struct Variables {
    int_variables: HashMap<String, i32>,
    float_variables: HashMap<String, f64>,
    string_variables: HashMap<String, String>
}

impl Variables {
    pub fn new() -> Self {
        Variables {
            int_variables: HashMap::new(),
            float_variables: HashMap::new(),
            string_variables: HashMap::new(),
        }
    }

    pub fn add_data_to_int(&mut self, k: String, v: i32) {
        self.int_variables.insert(k, v);
    }

    pub fn add_data_to_float(&mut self, k: String, v: f64) {
        self.float_variables.insert(k, v);
    }

    pub fn add_data_to_string(&mut self, k: String, v: String) {
        self.string_variables.insert(k, v);
    }

    pub fn get_int_data(&self, k: &str) -> Result<i32, String> {
        match self.int_variables.get(k) {
            Some(&value) => Ok(value),
            None => Err(format!("Error: can't found variable {}", k)),        
        }
    }

    pub fn get_float_data(&self, k: &str) -> Result<f64, String> {
        match self.float_variables.get(k) {
            Some(&value) => Ok(value),
            None => Err(format!("Error: can't found variable {}", k)),        
        }
    }

    pub fn get_string_data(&self, k: &str) -> Result<String, String> {
        match self.string_variables.get(k) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Error: can't found variable {}", k)),        
        }
    }
}

// Global variable store (thread-safe)
static VARIABLE_STORE: OnceLock<Mutex<Variables>> = OnceLock::new();

fn get_variable_store() -> &'static Mutex<Variables> {
    VARIABLE_STORE.get_or_init(|| Mutex::new(Variables::new()))
}

fn help() {
    println!("{}", "get_random_num : use for get random num with diapason".blue());
    println!("Examples:");
    println!(" get_random_num 1 100    - generates random integer between 1-100");
    println!(" get_random_num 0.5 5.5  - generates random float between 0.5-5.5");
    
    println!("{}", "get_random_char : use for get random char from alphabet".blue());
    println!("Examples:");
    println!(" get_random_char      - return random lowercase char example 'a'");
    println!(" get_random_char 1    - return random uppercase char example 'B'");
    
    println!("{}", "repeat : use for repeat one command n times".blue());
    println!("Examples:");
    println!(" repeat 10 get_random_num 1 100 - repeat command get_random_num 10 times");
    
    println!("{}", "set : use for set variable with value".blue());
    println!("Examples:");
    println!(" set my_var 42          - set integer variable");
    println!(" set my_var 3.14        - set float variable");
    println!(" set my_var \"hello\"     - set string variable");
    println!(" set my_var $(get_random_num 1 100) - set with command result");
    
    println!("{}", "print : use for print variable value or literal".blue());
    println!("Examples:");
    println!(" print my_var          - print variable value");
    println!(" print \"Hello World\"   - print literal string");
    println!(" print 123            - print literal number");
    
    println!("{}", "exit/quit : exit the program".blue());
    println!("Examples:");
    println!(" exit                 - exit with code 0");
    println!(" exit 1              - exit with code 1");
    
    println!("{}", "help : show this help message".blue());
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

fn execute_command(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.is_empty() {
        // always return a String (empty) as success value
        return Ok(String::new());
    }

    match args[0].as_str() {
        "get_random_num" => {
            if args.len() != 3 {
                return Err(format!("Usage: get_random_num <min> <max>"));
            }

            // Try parsing as int
            if let (Ok(min), Ok(max)) = (args[1].parse::<i32>(), args[2].parse::<i32>()) {
                if min >= max {
                    return Err("min must be less than max".to_string());
                }
                let n: i32 = get_random_num(min, max);
                return if capture_output {
                    Ok(n.to_string())
                } else {
                    println!("Random integer: {}", n);
                    Ok(String::new())
                };
            }

            // Try parsing as floats
            if let (Ok(min), Ok(max)) = (args[1].parse::<f64>(), args[2].parse::<f64>()) {
                if min >= max {
                    return Err("min must be less than max".to_string());
                }
                let n: f64 = get_random_num(min, max);
                return if capture_output {
                    Ok(n.to_string())
                } else {
                    println!("Random float: {}", n);
                    Ok(String::new())
                };
            }

            Err("Arguments must be numbers (integers or floats)".to_string())
        }

        "get_random_char" => {
            let mode = if args.len() == 2 {
                args[1].parse::<i32>().unwrap_or(0)
            } else {
                0
            };

            match get_random_char(mode) {
                Ok(c) => {
                    if capture_output {
                        Ok(c.to_string())
                    } else {
                        println!("{}", c);
                        Ok(String::new())
                    }
                }
                Err(e) => Err(e),
            }
        }

        "quit" | "exit" if !capture_output => {
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
        
        "help" if !capture_output => {
            help();
            Ok(String::new())
        }

        "repeat" if !capture_output => {
            if args.len() < 3 {
                return Err("Usage: repeat <count> <command> [args...]".to_string());
            }

            let repeat_count: usize = match args[1].parse::<usize>() {
                Ok(n) => n,
                Err(e) => return Err(format!("Error: {}", e)),
            };

            let args_for_repeat: Vec<String> = args[2..].to_vec();

            for i in 0..repeat_count {
                if let Err(e) = execute_command(&args_for_repeat, false) {
                    return Err(format!("Error in iteration {}: {}", i + 1, e));
                }
            }
            println!("");

            Ok(String::new())
        }

        "set" if !capture_output => {
            if args.len() < 3 {
                return Err("Usage: set <name> <value>".to_string());
            }

            let name = args[1].clone();
            let raw_value = args[2..].join(" ");

            // Check if value starts with $( - command substitution
            if raw_value.starts_with("$(") && raw_value.ends_with(')') {
                let command_content = &raw_value[2..raw_value.len()-1];
                let command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&command_args, true) {
                    Ok(result) => {
                        // Try to determine the type and store appropriately
                        if let Ok(iv) = result.parse::<i32>() {
                            let mut store = get_variable_store().lock().unwrap();
                            store.add_data_to_int(name.clone(), iv);
                            println!("{}", format!("Debug: Set {} = {} (int) from command {}", name, iv, command_args[0]).custom_color((95, 12, 204)));
                            return Ok(String::new());
                        }

                        if let Ok(fv) = result.parse::<f64>() {
                            let mut store = get_variable_store().lock().unwrap();
                            store.add_data_to_float(name.clone(), fv);
                            println!("{}", format!("Debug: Set {} = {} (float) from command {}", name, fv, command_args[0]).custom_color((95, 12, 204)));
                            return Ok(String::new());
                        }

                        // Store as string
                        let mut store = get_variable_store().lock().unwrap();
                        store.add_data_to_string(name.clone(), result.clone());
                        println!("{}", format!("Debug: Set {} = '{}' (string) from command {}", name, result, command_args[0]).custom_color((95, 12, 204)));
                        return Ok(String::new());
                    }
                    Err(e) => return Err(format!("Error executing command: {}", e)),
                }
            }

            if let Ok(iv) = raw_value.parse::<i32>() {
                let mut store = get_variable_store().lock().unwrap();
                store.add_data_to_int(name.clone(), iv);
                println!("{}", format!("Set {} = {} (int)", name, iv).green());
                return Ok(String::new());
            }

            // try float
            if let Ok(fv) = raw_value.parse::<f64>() {
                let mut store = get_variable_store().lock().unwrap();
                store.add_data_to_float(name.clone(), fv);
                println!("{}", format!("Set {} = {} (float)", name, fv).green());
                return Ok(String::new());
            }

            // otherwise store as string (strip surrounding quotes if present)
            let mut sv = raw_value.clone();
            if sv.len() >= 2 {
                if (sv.starts_with('"') && sv.ends_with('"')) || (sv.starts_with('\'') && sv.ends_with('\'')) {
                    sv = sv[1..sv.len()-1].to_string();
                }
            }
            let mut store = get_variable_store().lock().unwrap();
            store.add_data_to_string(name.clone(), sv.clone());
            println!("{}", format!("Set {} = '{}' (string)", name, sv).green());
            Ok(String::new())
        }

        "print" if !capture_output => {
            // print <name or literal>
            if args.len() < 2 {
                return Err("Usage: print <name or literal>".to_string());
            }

            let name = args[1].clone();

            // try to find variable
            let store = get_variable_store().lock().unwrap();

            if let Ok(iv) = store.get_int_data(&name) {
                println!("{}", format!("{} = {}", name, iv).yellow());
                return Ok(String::new());
            }

            if let Ok(fv) = store.get_float_data(&name) {
                println!("{}", format!("{} = {}", name, fv).yellow());
                return Ok(String::new());
            }

            if let Ok(sv) = store.get_string_data(&name) {
                println!("{}", format!("{} = '{}'", name, sv).yellow());
                return Ok(String::new());
            }

            // If variable not found, print the literal
            println!("{}", format!("{}", name).yellow());
            Ok(String::new())
        }

        _ => {
            if capture_output {
                Err(format!("Command '{}' cannot be used in variable assignment", args[0]))
            } else {
                Err(format!("Unknown command {}", args[0]))
            }
        }
    }
}
pub fn interpret_arguments_from_command_line(args: &[String]) -> Result<(), String> {
    match execute_command(args, false) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
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