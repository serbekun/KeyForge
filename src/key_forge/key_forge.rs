use rand::Rng;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use colored::*;
use std::collections::HashMap;
use std::sync::{Mutex};
use std::sync::OnceLock;

use crate::utils;
use super::execute_command::execute_command;

pub static FILE_STACK: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
pub fn get_file_stack() -> &'static Mutex<Vec<String>> {
    FILE_STACK.get_or_init(|| Mutex::new(Vec::new()))
}

#[derive(Debug)]
pub enum ParsedValue {
    Int(i32),
    Float(f64),
    String(String),
}

pub struct Variables {
    pub int_variables: HashMap<String, i32>,
    pub float_variables: HashMap<String, f64>,
    pub string_variables: HashMap<String, String>
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
pub static VARIABLE_STORE: OnceLock<Mutex<Variables>> = OnceLock::new();

pub fn get_variable_store() -> &'static Mutex<Variables> {
    VARIABLE_STORE.get_or_init(|| Mutex::new(Variables::new()))
}

// Unified function for parsing values with type detection
pub fn parse_value(value: &str) -> ParsedValue {
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
pub fn store_parsed_value(name: String, value: ParsedValue, from_command: Option<&str>) -> Result<(), String> {
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

pub fn get_random_char(mode: i32) -> Result<char, String> {
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

pub fn get_random_num<T: rand::distributions::uniform::SampleUniform + std::cmp::PartialOrd>(
    min: T,
    max: T,
) -> T {
    rand::thread_rng().gen_range(min..=max)
}

// Helper function to check if a string is a valid variable identifier
pub fn is_valid_identifier(s: &str) -> bool {
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
pub fn perform_arithmetic(operation: &str, var_name: &str, value: ParsedValue) -> Result<(), String> {
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

pub fn tokenize_input(input: &str) -> Vec<String> {
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

        if let Err(e) = interpret_arguments_from_command_line(&args) {
            println!("{}", format!("in line {}: {}", line_num, e).red().bold());
            println!("{}", "Stop interpret program".red().bold());
            break;
        }
    }

    let mut stack = get_file_stack().lock().unwrap();
    stack.pop();
}