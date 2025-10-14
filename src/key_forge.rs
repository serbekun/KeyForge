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

static FILE_STACK: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
fn get_file_stack() -> &'static Mutex<Vec<String>> {
    FILE_STACK.get_or_init(|| Mutex::new(Vec::new()))
}

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

    // variables list
    pub fn vl(&self, mode: &str) {
        fn print_section<T: std::fmt::Display>(title: &str, vars: &HashMap<String, T>, suffix: &str) {
            println!("{}" ,format!("{}", title).green());
            for (key, value) in vars {
                println!("{}", format!("{}: {}{}", key, value, suffix).yellow());
            }
        }

        match mode {
            "i" => print_section("=== Integer Variables (i32) ===", &self.int_variables, " (i32)"),
            "f" => print_section("=== Float Variables (f64) ===", &self.float_variables, " (f64)"),
            "s" => print_section("=== String Variables (String) ===", &self.string_variables, " (String)"),
            _ => {
                print_section("=== Integer Variables (i32) ===", &self.int_variables, " (i32)");
                println!();
                print_section("=== Float Variables (f64) ===", &self.float_variables, " (f64)");
                println!();
                print_section("=== String Variables (String) ===", &self.string_variables, " (String)");
            }
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

    pub fn remove_int_data(&mut self, k: &String) {
        self.int_variables.remove(k);
    }

    pub fn remove_float_data(&mut self, k: &String) {
        self.float_variables.remove(k);
    }

    pub fn remove_string_data(&mut self, k: &String) {
        self.string_variables.remove(k);
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
    println!("{}", "Commands of key_forge".green());
    println!("");

    println!("{}", "get_random_num : use for get random num with diapason".blue());
    println!("Examples:");
    println!(" get_random_num 1 100    - generates random integer between 1-100");
    println!(" get_random_num 0.5 5.5  - generates random float between 0.5-5.5");
    println!("");

    println!("{}", "get_random_char : use for get random char from alphabet".blue());
    println!("Examples:");
    println!(" get_random_char      - return random lowercase char example 'a'");
    println!(" get_random_char 1    - return random uppercase char example 'B'");
    println!("");

    println!("{}", "repeat : use for repeat one command n times".blue());
    println!("Examples:");
    println!(" repeat 10 get_random_num 1 100 - repeat command get_random_num 10 times");
    println!("");

    println!("{}", "set : use for set variable with value".blue());
    println!("Examples:");
    println!(" set my_var 42                      - set integer variable");
    println!(" set my_var 3.14                    - set float variable");
    println!(" set my_var \"hello\"               - set string variable");
    println!(" set my_var $(get_random_num 1 100) - set with command result");
    println!("");

    println!("{}", "print : use for print variable value or literal".blue());
    println!("Examples:");
    println!(" print my_var                  - print variable value");
    println!(" print \"Hello World\"         - print literal string");
    println!(" print 123                     - print literal number");
    println!(" print $(get_random_num 1 100) - print output command get_random_num");
    println!("");
    

    println!("{}", "exit/quit : exit the program".blue());
    println!("Examples:");
    println!(" exit                 - exit with code 0");
    println!(" exit 1              - exit with code 1");
    println!("");

    println!("{}", "vl : use for show variables list".blue());
    println!("Examples:");
    println!(" vl - show all variables");
    println!(" vl i - show only int variables");
    println!(" vl f - show only float variables");
    println!(" vl s - show only string variables");
    println!("");

    println!("{}", "execute_file : for execute commands in file".blue());
    println!("Examples:");
    println!(" execute_file filename.txt - execute command in file filename.txt");
    println!("");

    println!("{}", "to_file : use for write output to file".blue());
    println!("Examples:");
    println!(" to_file filename.txt $(get_random_num 1 100) - write to file output of get_random_num");
    println!("");

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
        "//" => {
            Ok(String::new())
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
                let n: i32 = get_random_num(min, max);
                return if capture_output {
                    Ok(n.to_string())
                } else {
                    println!("{}", n);
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
                    println!("{}", n);
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

        "repeat" => {
            if args.len() < 3 {
                return Err("Usage: repeat <count> <command...>".to_string());
            }

            let count: usize = args[1].parse().unwrap_or(0);
            let raw_value = args[2..].join(" ");

            let mut results = Vec::new();

            if raw_value.starts_with("$(") && raw_value.ends_with(')') {
                let command_content = &raw_value[2..raw_value.len()-1];
                let command_args: Vec<String> = tokenize_input(command_content);

                for _ in 0..count {
                    match execute_command(&command_args, true) {
                        Ok(res) => {
                            if capture_output {
                                results.push(res);
                            } else {
                                println!("{}", res); 
                            }
                        }
                        Err(e) => return Err(format!("Error executing inner command: {}", e)),
                    }
                }
            }

            if capture_output {
                Ok(results.join("\n"))
            } else {
                Ok(String::new())
            }
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

        "rm" => {
            let mut store = get_variable_store().lock().unwrap();
            let k = &args[1];

            if store.int_variables.contains_key(k) {
                store.remove_int_data(k);
                return Ok(String::new());
            }

            if store.float_variables.contains_key(k) {
                store.remove_float_data(k);
                return Ok(String::new());
            }

            if store.string_variables.contains_key(k) {
                store.remove_string_data(k);
                return Ok(String::new());
            }

            Err(format!("Variable {} nit found", k))
        }

        "print" => {
            if args.len() < 2 {
                return Err("Usage: print <name or literal>".to_string());
            }

            let raw_value = args[1..].join(" ");

            if raw_value.starts_with("$(") && raw_value.ends_with(')') {
                let command_content = &raw_value[2..raw_value.len() - 1];
                let command_args: Vec<String> = tokenize_input(command_content);

                match execute_command(&command_args, capture_output) {
                    Ok(result) => {
                        if capture_output {
                            return Ok(result);
                        } else {
                            println!("{}", result);
                            return Ok(String::new());
                        }
                    }
                    Err(e) => return Err(format!("Error executing command: {}", e)),
                }
            }

            let variable_name = args[1].clone();
            let store = get_variable_store().lock().unwrap();

            if let Ok(iv) = store.get_int_data(&variable_name) {
                if capture_output {
                    return Ok(iv.to_string());
                } else {
                    println!("{}", iv);
                    return Ok(String::new());
                }
            }

            if let Ok(fv) = store.get_float_data(&variable_name) {
                if capture_output {
                    return Ok(fv.to_string());
                } else {
                    println!("{}", fv);
                    return Ok(String::new());
                }
            }

            if let Ok(sv) = store.get_string_data(&variable_name) {
                if capture_output {
                    return Ok(sv);
                } else {
                    println!("{}", sv);
                    return Ok(String::new());
                }
            }

            if capture_output {
                Ok(raw_value)
            } else {
                println!("{}", raw_value);
                Ok(String::new())
            }
        }

        "execute_file" => {
            if args.len() < 2 {
                return Err(format!("Usage: execute_file <filename>"));
            }

            file_mode(&args[1]);
            Ok(String::new())
        }

        "vl" => {
            let mode = if args.len() >= 2 { args[1].as_str() } else { "" };
            let store = get_variable_store().lock().unwrap();

            if capture_output {
                use std::fmt::Write;
                let mut output = String::new();

                fn collect_section<T: std::fmt::Display>(
                    output: &mut String,
                    title: &str,
                    vars: &HashMap<String, T>,
                    suffix: &str,
                ) {
                    writeln!(output, "{}", title).unwrap();
                    for (k, v) in vars {
                        writeln!(output, "{}: {}{}", k, v, suffix).unwrap();
                    }
                    writeln!(output).unwrap();
                }

                match mode {
                    "i" => collect_section(&mut output, "=== Integer Variables (i32) ===", &store.int_variables, " (i32)"),
                    "f" => collect_section(&mut output, "=== Float Variables (f64) ===", &store.float_variables, " (f64)"),
                    "s" => collect_section(&mut output, "=== String Variables (String) ===", &store.string_variables, " (String)"),
                    _ => {
                        collect_section(&mut output, "=== Integer Variables (i32) ===", &store.int_variables, " (i32)");
                        collect_section(&mut output, "=== Float Variables (f64) ===", &store.float_variables, " (f64)");
                        collect_section(&mut output, "=== String Variables (String) ===", &store.string_variables, " (String)");
                    }
                }

                Ok(output)
            } else {
                store.vl(mode);
                Ok(String::new())
            }
        }

        "to_file" => {
            if args.len() < 3 {
                return Err("Usage: to_file <filename> <command...>".to_string());
            }

            let filename = &args[1];
            let command_args = &args[2..];

            // Handle command substitution
            let raw_command = command_args.join(" ");
            if raw_command.starts_with("$(") && raw_command.ends_with(')') {
                let command_content = &raw_command[2..raw_command.len()-1];
                let inner_command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&inner_command_args, true) {
                    Ok(output) => {
                        use std::fs::OpenOptions;
                        use std::io::Write;

                        let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(filename)
                            .map_err(|e| format!("Error opening file '{}': {}", filename, e))?;

                        writeln!(file, "{}", output)
                            .map_err(|e| format!("Error writing to file '{}': {}", filename, e))?;
                        
                        println!("{}", format!("Output written to file '{}'", filename).green());
                        Ok(String::new())
                    }
                    Err(e) => return Err(format!("Error executing inner command: {}", e)),
                }
            } else {
                // If it's not a command substitution, execute the command directly
                match execute_command(command_args, true) {
                    Ok(output) => {
                        use std::fs::OpenOptions;
                        use std::io::Write;

                        let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(filename)
                            .map_err(|e| format!("Error opening file '{}': {}", filename, e))?;

                        writeln!(file, "{}", output)
                            .map_err(|e| format!("Error writing to file '{}': {}", filename, e))?;
                        
                        println!("{}", format!("Output written to file '{}'", filename).green());
                        Ok(String::new())
                    }
                    Err(e) => return Err(format!("Error executing command: {}", e)),
                }
            }
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
    {
        let mut stack = get_file_stack().lock().unwrap();
        if stack.contains(filename) {
            eprintln!("{}", format!("Recursive include detected for file '{}', skipping to avoid stack overflow.", filename).red().bold());
            return;
        }
        stack.push(filename.clone());
    }

    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", format!("Error open file {}: {}", filename, e).red().bold());
            let mut stack = get_file_stack().lock().unwrap();
            stack.pop();
            return;
        }
    };

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
            println!("{}", "Stop interpret program".red().bold());
            break;
        }
    }

    // pop file from stack
    let mut stack = get_file_stack().lock().unwrap();
    stack.pop();
}