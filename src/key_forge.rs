use rand::Rng;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use colored::*;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
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

    pub fn has_variable(&self, name: &str) -> bool {
        self.int_variables.contains_key(name) || self.float_variables.contains_key(name) || self.string_variables.contains_key(name)
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

// Enum to represent parsed value types
#[derive(Debug)]
enum ParsedValue {
    Int(i32),
    Float(f64),
    String(String),
}

// Unified function for parsing values with type detection
fn parse_value(value: &str) -> ParsedValue {
    // Try parsing as integer first
    if let Ok(int_val) = value.parse::<i32>() {
        return ParsedValue::Int(int_val);
    }
    
    // Try parsing as float
    if let Ok(float_val) = value.parse::<f64>() {
        return ParsedValue::Float(float_val);
    }
    
    // Otherwise treat as string (strip surrounding quotes if present)
    let mut string_val = value.to_string();
    if string_val.len() >= 2 {
        if (string_val.starts_with('"') && string_val.ends_with('"')) || 
           (string_val.starts_with('\'') && string_val.ends_with('\'')) {
            string_val = string_val[1..string_val.len()-1].to_string();
        }
    }
    
    ParsedValue::String(string_val)
}

// Helper function to store parsed value in variables
fn store_parsed_value(name: String, value: ParsedValue, from_command: Option<&str>) -> Result<(), String> {
    let mut store = get_variable_store().lock().unwrap();
    
    match value {
        ParsedValue::Int(i) => {
            store.add_data_to_int(name.clone(), i);
            if let Some(cmd) = from_command {
                println!("{}", format!("Debug: Set {} = {} (int) from command {}", name, i, cmd).custom_color((95, 12, 204)));
            } else {
                println!("{}", format!("Set {} = {} (int)", name, i).green());
            }
        }
        ParsedValue::Float(f) => {
            store.add_data_to_float(name.clone(), f);
            if let Some(cmd) = from_command {
                println!("{}", format!("Debug: Set {} = {} (float) from command {}", name, f, cmd).custom_color((95, 12, 204)));
            } else {
                println!("{}", format!("Set {} = {} (float)", name, f).green());
            }
        }
        ParsedValue::String(s) => {
            store.add_data_to_string(name.clone(), s.clone());
            if let Some(cmd) = from_command {
                println!("{}", format!("Debug: Set {} = '{}' (string) from command {}", name, s, cmd).custom_color((95, 12, 204)));
            } else {
                println!("{}", format!("Set {} = '{}' (string)", name, s).green());
            }
        }
    }
    
    Ok(())
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

    println!("{}", "add : for add value to variable".blue());
    println!("Examples:");
    println!("add x 10                      - add 10 to variable x");
    println!("add x y                       - add other to x");
    println!("add x $(get_random_num 1 100) - add output of get_random_num");
    println!();
    
    println!("{}", "mul : for multiply values".blue());
    println!("Examples:");
    println!("mul x 5                          - multiply x * 3 and store in x");
    println!("mul x y                          - multiply variables x y and store in x");
    println!("mul result $(get_random_num 2 5) - multiply random number by 10");
    println!("");

    println!("{}", "div : for divide values".blue());
    println!("Examples:");
    println!("div x 2                              - divide 10 / 2 and store in x");
    println!("div x z                              - divide y / z and store in x");
    println!("div result 100 $(get_random_num 2 5) - divide 100 by random number");
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
        return Ok(String::new());
    }

    match args[0].as_str() {
        "//" => Ok(String::new()),

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
                        let parsed_value = parse_value(&result);
                        store_parsed_value(name, parsed_value, Some(&command_args[0]))?;
                        Ok(String::new())
                    }
                    Err(e) => Err(format!("Error executing command: {}", e)),
                }
            } else {
                // Direct value assignment
                let parsed_value = parse_value(&raw_value);
                store_parsed_value(name, parsed_value, None)?;
                Ok(String::new())
            }
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

            Err(format!("Variable {} not found", k))
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

            let variable_name: String = args[1].clone();
            let store: MutexGuard<'_, Variables> = get_variable_store().lock().unwrap();

            if let Ok(iv) = store.get_int_data(&variable_name) {
                if capture_output {
                    return Ok(iv.to_string());
                } else {
                    println!("{}", format!("{}", iv).yellow());
                    return Ok(String::new());
                }
            }

            if let Ok(fv) = store.get_float_data(&variable_name) {
                if capture_output {
                    return Ok(fv.to_string());
                } else {
                    println!("{}", format!("{}", fv).yellow());
                    return Ok(String::new());
                }
            }

            if let Ok(sv) = store.get_string_data(&variable_name) {
                if capture_output {
                    return Ok(sv);
                } else {
                    println!("{}", format!("{}", sv).yellow());
                    return Ok(String::new());
                }
            }

            if capture_output {
                Ok(raw_value)
            } else {
                println!("{}" ,format!("{}", raw_value).yellow());
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
            let mode: &str = if args.len() >= 2 { args[1].as_str() } else { "" };
            let store: MutexGuard<'_, Variables> = get_variable_store().lock().unwrap();

            if capture_output {
                use std::fmt::Write;
                let mut output: String = String::new();

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

                        let mut file: File = OpenOptions::new()
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

        "add" | "sub" | "mul" | "div" => {
            let command_args = &args[2..];
            let raw_command = command_args.join(" ");
            
            if raw_command.starts_with("$(") && raw_command.ends_with(')') {
                let command_content = &raw_command[2..raw_command.len()-1];
                let inner_command_args: Vec<String> = tokenize_input(command_content);
                
                match execute_command(&inner_command_args, true) {
                    Ok(output) => {
                        let parsed_value = parse_value(&output);
                        perform_arithmetic(&args[0], &args[1], parsed_value)?;
                        Ok(String::new())
                    }
                    Err(e) => Err(format!("Error executing inner command: {}", e)),
                }            
            } else {
                // Parse the value - it could be a direct value or a variable name
                let parsed_value = if is_valid_identifier(&raw_command) {
                    // Try to get value from variable
                    let store = get_variable_store().lock().unwrap();
                    if store.has_variable(&raw_command) {
                        // Get the value from the variable
                        if let Ok(int_val) = store.get_int_data(&raw_command) {
                            ParsedValue::Int(int_val)
                        } else if let Ok(float_val) = store.get_float_data(&raw_command) {
                            ParsedValue::Float(float_val)
                        } else if let Ok(string_val) = store.get_string_data(&raw_command) {
                            ParsedValue::String(string_val)
                        } else {
                            parse_value(&raw_command)
                        }
                    } else {
                        parse_value(&raw_command)
                    }
                } else {
                    parse_value(&raw_command)
                };
                
                perform_arithmetic(&args[0], &args[1], parsed_value)?;
                Ok(String::new())
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
// Helper function to check if a string is a valid variable identifier
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    // Check if first character is alphabetic or underscore
    let first_char = s.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }
    
    // Check if all characters are alphanumeric or underscore
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

// Helper function for arithmetic operations
fn perform_arithmetic(operation: &str, var_name: &str, value: ParsedValue) -> Result<(), String> {
    let mut store = get_variable_store().lock().unwrap();
    
    match operation {
        "add" => {
            match value {
                ParsedValue::Int(iv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_int(var_name.to_string(), val + iv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val + iv as f64);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_string_data(var_name) {
                        store.add_data_to_string(var_name.to_string(), val + &iv.to_string());
                        return Ok(());
                    }
                }
                ParsedValue::Float(fv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val as f64 + fv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val + fv);
                        return Ok(());
                    }
                }
                ParsedValue::String(sv) => {
                    if let Ok(val) = store.get_string_data(var_name) {
                        store.add_data_to_string(var_name.to_string(), val + &sv);
                        return Ok(());
                    }
                }
            }
            Err(format!("Variable {} not found or incompatible type", var_name))
        }
        "sub" => {
            match value {
                ParsedValue::Int(iv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_int(var_name.to_string(), val - iv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val - iv as f64);
                        return Ok(());
                    }
                }
                ParsedValue::Float(fv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val as f64 - fv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val - fv);
                        return Ok(());
                    }
                }
                _ => return Err("Cannot subtract non-numeric value".to_string()),
            }
            Err(format!("Variable {} not found or not a number", var_name))
        }
        "mul" => {
            match value {
                ParsedValue::Int(iv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_int(var_name.to_string(), val * iv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val * iv as f64);
                        return Ok(());
                    }
                }
                ParsedValue::Float(fv) => {
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val as f64 * fv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val * fv);
                        return Ok(());
                    }
                }
                _ => return Err("Cannot multiply by non-numeric value".to_string()),
            }
            Err(format!("Variable {} not found or not a number", var_name))
        }
        "div" => {
            match value {
                ParsedValue::Int(iv) => {
                    if iv == 0 {
                        return Err("Division by zero".to_string());
                    }
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_int(var_name.to_string(), val / iv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val / iv as f64);
                        return Ok(());
                    }
                }
                ParsedValue::Float(fv) => {
                    if fv == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    if let Ok(val) = store.get_int_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val as f64 / fv);
                        return Ok(());
                    }
                    if let Ok(val) = store.get_float_data(var_name) {
                        store.add_data_to_float(var_name.to_string(), val / fv);
                        return Ok(());
                    }
                }
                _ => return Err("Cannot divide by non-numeric value".to_string()),
            }
            Err(format!("Variable {} not found or not a number", var_name))
        }
        _ => Err(format!("Unknown operation: {}", operation)),
    }
}

// The rest of the functions remain the same...
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
                    "" => continue,
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

    let mut stack = get_file_stack().lock().unwrap();
    stack.pop();
}