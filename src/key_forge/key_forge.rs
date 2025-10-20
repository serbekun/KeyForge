use colored::Colorize;
use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, BufRead};
use std::sync::Mutex;

use crate::utils;

#[derive(Debug, Clone)]
pub enum ParsedValue {
    Int(i32),
    Float(f64),
    String(String),
}

#[derive(Debug)]
pub struct Variables {
    pub int_variables: HashMap<String, i32>,
    pub float_variables: HashMap<String, f64>,
    pub string_variables: HashMap<String, String>,
}

impl Variables {
    pub fn new() -> Self {
        Self {
            int_variables: HashMap::new(),
            float_variables: HashMap::new(),
            string_variables: HashMap::new(),
        }
    }

    pub fn has_variable(&self, name: &str) -> bool {
        self.int_variables.contains_key(name)
            || self.float_variables.contains_key(name)
            || self.string_variables.contains_key(name)
    }

    pub fn get_int_data(&self, name: &str) -> Result<i32, String> {
        self.int_variables
            .get(name)
            .copied()
            .ok_or_else(|| format!("Int variable '{}' not found", name))
    }

    pub fn get_float_data(&self, name: &str) -> Result<f64, String> {
        self.float_variables
            .get(name)
            .copied()
            .ok_or_else(|| format!("Float variable '{}' not found", name))
    }

    pub fn get_string_data(&self, name: &str) -> Result<String, String> {
        self.string_variables
            .get(name)
            .cloned()
            .ok_or_else(|| format!("String variable '{}' not found", name))
    }

    pub fn add_data_to_int(&mut self, name: String, v: i32) {
        self.int_variables.insert(name, v);
    }

    pub fn add_data_to_float(&mut self, name: String, v: f64) {
        self.float_variables.insert(name, v);
    }

    pub fn add_data_to_string(&mut self, name: String, v: String) {
        self.string_variables.insert(name, v);
    }

    pub fn remove_int_data(&mut self, name: &str) {
        self.int_variables.remove(name);
    }

    pub fn remove_float_data(&mut self, name: &str) {
        self.float_variables.remove(name);
    }

    pub fn remove_string_data(&mut self, name: &str) {
        self.string_variables.remove(name);
    }

    pub fn vl(&self, mode: &str) {
        match mode {
            "i" => {
                println!("=== Integer Variables (i32) ===");
                for (k, v) in &self.int_variables {
                    println!("{}: {}", k, v);
                }
            }
            "f" => {
                println!("=== Float Variables (f64) ===");
                for (k, v) in &self.float_variables {
                    println!("{}: {}", k, v);
                }
            }
            "s" => {
                println!("=== String Variables (String) ===");
                for (k, v) in &self.string_variables {
                    println!("{}: {}", k, v);
                }
            }
            _ => {
                println!("=== Integer Variables (i32) ===");
                for (k, v) in &self.int_variables {
                    println!("{}: {}", k, v);
                }
                println!("");
                println!("=== Float Variables (f64) ===");
                for (k, v) in &self.float_variables {
                    println!("{}: {}", k, v);
                }
                println!("");
                println!("=== String Variables (String) ===");
                for (k, v) in &self.string_variables {
                    println!("{}: {}", k, v);
                }
            }
        }
    }
}

lazy_static! {
    static ref VARIABLE_STORE: Mutex<Variables> = Mutex::new(Variables::new());
    static ref BREAK_FLAG: Mutex<bool> = Mutex::new(false);
    static ref CONTINUE_FLAG: Mutex<bool> = Mutex::new(false);
}

pub fn set_break_flag(value: bool) {
    let mut flag = BREAK_FLAG.lock().unwrap();
    *flag = value;
}

pub fn set_continue_flag(value: bool) {
    let mut flag = CONTINUE_FLAG.lock().unwrap();
    *flag = value;
}

pub fn should_break() -> bool {
    let flag = BREAK_FLAG.lock().unwrap();
    *flag
}

pub fn should_continue() -> bool {
    let flag = CONTINUE_FLAG.lock().unwrap();
    *flag
}

pub fn reset_loop_flags() {
    set_break_flag(false);
    set_continue_flag(false);
}

pub fn get_variable_store() -> &'static Mutex<Variables> {
    &*VARIABLE_STORE
}

pub fn parse_value(raw: &str) -> ParsedValue {
    if let Ok(iv) = raw.parse::<i32>() {
        return ParsedValue::Int(iv);
    }

    if let Ok(fv) = raw.parse::<f64>() {
        return ParsedValue::Float(fv);
    }

    let s = raw.trim();
    let s = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).unwrap_or(s);
    let s = s.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')).unwrap_or(s);

    ParsedValue::String(s.to_string())
}

pub fn store_parsed_value(name: String, value: ParsedValue, _source: Option<&str>) -> Result<(), String> {
    let mut store = get_variable_store().lock().map_err(|e| format!("Mutex poisoned: {}", e))?;

    match value {
        ParsedValue::Int(iv) => store.add_data_to_int(name, iv),
        ParsedValue::Float(fv) => store.add_data_to_float(name, fv),
        ParsedValue::String(sv) => store.add_data_to_string(name, sv),
    }

    Ok(())
}

// Extract variable resolution into a helper function
pub fn resolve_to_string(value: &str) -> Result<String, String> {
    // Allow using $var or var to reference variables
    let key = if value.starts_with('$') { &value[1..] } else { value };

    let store = get_variable_store().lock().unwrap();

    if let Ok(int_val) = store.get_int_data(key) {
        Ok(int_val.to_string())
    } else if let Ok(float_val) = store.get_float_data(key) {
        Ok(float_val.to_string())
    } else if let Ok(string_val) = store.get_string_data(key) {
        Ok(string_val)
    } else {
        // Variable doesn't exist - try parsing as literal
        let parsed_value = parse_value(value);
        match parsed_value {
            ParsedValue::Int(i) => Ok(i.to_string()),
            ParsedValue::Float(f) => Ok(f.to_string()),
            ParsedValue::String(s) => Ok(s),
        }
    }
}

pub fn tokenize_input(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';

    for c in input.chars() {
        if in_quotes {
            if c == quote_char {
                in_quotes = false;
                continue;
            }
            current.push(c);
        } else {
            if c == '"' || c == '\'' {
                in_quotes = true;
                quote_char = c;
                continue;
            }
            if c.is_whitespace() {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

pub fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        if !first.is_alphabetic() && first != '_' {
            return false;
        }
    } else {
        return false;
    }

    chars.all(|c| c.is_alphanumeric() || c == '_')
}

pub fn get_random_char(mode: i32) -> Result<char, String> {
    let mut rng = rand::thread_rng();
    let base = if mode == 1 { 'A' as u8 } else { 'a' as u8 };
    let offset = rng.gen_range(0..26);
    Ok((base + offset) as char)
}

// Generic wrapper used by execute_command which expects `get_random_num` generic
pub fn get_random_num<T>(min: T, max: T) -> T
where
    T: PartialOrd + Copy + rand::distributions::uniform::SampleUniform,
{
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn substitute_variables_in_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '$' {
            // Start reading variable name
            let mut var_name = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    var_name.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            
            if !var_name.is_empty() {
                // Try to get variable value
                let store = get_variable_store().lock().unwrap();
                if let Ok(int_val) = store.get_int_data(&var_name) {
                    result.push_str(&int_val.to_string());
                } else if let Ok(float_val) = store.get_float_data(&var_name) {
                    result.push_str(&float_val.to_string());
                } else if let Ok(string_val) = store.get_string_data(&var_name) {
                    result.push_str(&string_val);
                } else {
                    // Variable not found, leave as is
                    result.push('$');
                    result.push_str(&var_name);
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

pub fn evaluate_condition(condition: &str) -> Result<bool, String> {
    let tokens: Vec<&str> = condition.split_whitespace().collect();

    if tokens.len() >= 3 && (tokens.contains(&"and") || tokens.contains(&"or")) {
        return evaluate_complex_condition(&tokens);
    }

    if tokens.len() < 3 {
        return Err("Condition must have at least 3 parts".to_string());
    }

    let left = resolve_to_string(tokens[0])?;
    let operator = tokens[1];
    let right = resolve_to_string(&tokens[2..].join(" "))?;

    if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
        match operator {
            "==" | "eq" => Ok((left_num - right_num).abs() < f64::EPSILON),
            "!=" | "ne" => Ok((left_num - right_num).abs() > f64::EPSILON),
            ">" | "gt" => Ok(left_num > right_num),
            "<" | "lt" => Ok(left_num < right_num),
            ">=" | "ge" => Ok(left_num >= right_num),
            "<=" | "le" => Ok(left_num <= right_num),
            _ => Err(format!("Unknown operator: {}", operator)),
        }
    } else {
        match operator {
            "==" | "eq" => Ok(left == right),
            "!=" | "ne" => Ok(left != right),
            ">" | "gt" => Ok(left > right),
            "<" | "lt" => Ok(left < right),
            ">=" | "ge" => Ok(left >= right),
            "<=" | "le" => Ok(left <= right),
            _ => Err(format!("Unknown operator: {}", operator)),
        }
    }
}

fn evaluate_complex_condition(tokens: &[&str]) -> Result<bool, String> {
    let mut result = None;
    let mut current_operator = "and";
    
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "and" || tokens[i] == "or" {
            current_operator = tokens[i];
            i += 1;
            continue;
        }

        let mut condition_end = i;
        while condition_end < tokens.len() && tokens[condition_end] != "and" && tokens[condition_end] != "or" {
            condition_end += 1;
        }
        
        let condition_tokens = &tokens[i..condition_end];
        if condition_tokens.len() < 3 {
            return Err("Invalid condition in complex expression".to_string());
        }
        
        let simple_condition = condition_tokens.join(" ");
        let condition_result = evaluate_condition(&simple_condition)?;
        
        result = match result {
            None => Some(condition_result),
            Some(current) => match current_operator {
                "and" => Some(current && condition_result),
                "or" => Some(current || condition_result),
                _ => Some(condition_result),
            },
        };
        
        i = condition_end;
    }
    
    result.ok_or("No conditions found".to_string())
}

pub fn parse_block_commands(input: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut current = String::new();
    let mut brace_count = 0;
    let mut in_quotes = false;
    let mut quote_char = '\0';
    
    for c in input.chars() {
        if in_quotes {
            current.push(c);
            if c == quote_char {
                in_quotes = false;
            }
        } else {
            match c {
                '"' | '\'' => {
                    in_quotes = true;
                    quote_char = c;
                    current.push(c);
                }
                '{' => {
                    brace_count += 1;
                    current.push(c);
                }
                '}' => {
                    brace_count -= 1;
                    current.push(c);
                }
                ';' if brace_count == 0 => {
                    if !current.trim().is_empty() {
                        commands.push(current.trim().to_string());
                    }
                    current.clear();
                }
                _ => current.push(c),
            }
        }
    }
    
    if !current.trim().is_empty() {
        commands.push(current.trim().to_string());
    }
    
    commands
}

pub fn cli_mode() {
    println!("{}" ,"KeyForge CLI mode".green().bold());
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        print!("> ");
        utils::flush_stdout();
        buffer.clear();
        if stdin.lock().read_line(&mut buffer).is_err() {
            break;
        }
        let input = buffer.trim();
        if input.is_empty() {
            continue;
        }
        let args = tokenize_input(input);
        if args.is_empty() {
            continue;
        }
        // delegate to execute_command in sibling module
        match crate::key_forge::execute_command::execute_command(&args, false) {
            Ok(_) => continue,
            Err(e) => println!("{}" ,format!("Error: {}", e).red()),
        }
    }
}

pub fn file_mode(filename: &str) {
    if let Ok(file) = OpenOptions::new().read(true).open(filename) {
        let reader = io::BufReader::new(file);

        // We'll accumulate lines and handle multi-line blocks enclosed in braces { }
        let mut buffer = String::new();
        let mut brace_depth: i32 = 0;

        for line in reader.lines() {
            if let Ok(l) = line {
                // Preserve original line trimming for normal commands but keep spaces inside blocks
                let trimmed = l.trim().to_string();

                // If we're already inside a block, append the raw line (with a space) to buffer
                if brace_depth > 0 {
                    // Use semicolon to separate original lines inside a block so
                    // `parse_block_commands` (which splits on ';') will produce
                    // separate commands for each original line.
                    // Keep spaces around separators so tokenization preserves
                    // `{` and `}` as separate tokens.
                    if buffer.ends_with('{') {
                        // just add a space after the opening brace
                        buffer.push(' ');
                        buffer.push_str(trimmed.as_str());
                    } else {
                        // separate previous command and this one with ' ; '
                        buffer.push_str(" ; ");
                        buffer.push_str(trimmed.as_str());
                    }
                    // Update brace depth based on occurrences in this line
                    brace_depth += trimmed.matches('{').count() as i32;
                    brace_depth -= trimmed.matches('}').count() as i32;

                    if brace_depth <= 0 {
                        // End of block reached; execute the combined command
                        let args = tokenize_input(buffer.trim());
                        let _ = crate::key_forge::execute_command::execute_command(&args, false);
                        buffer.clear();
                        brace_depth = 0;
                    }
                    continue;
                }

                // Not currently in a block. Check if this line starts a block
                if trimmed.contains('{') {
                    // Start collecting block
                    buffer = trimmed.clone();
                    brace_depth += trimmed.matches('{').count() as i32;
                    brace_depth -= trimmed.matches('}').count() as i32;

                    if brace_depth <= 0 {
                        // Opening and closing brace on same line
                        let args = tokenize_input(buffer.trim());
                        let _ = crate::key_forge::execute_command::execute_command(&args, false);
                        buffer.clear();
                        brace_depth = 0;
                    }
                    continue;
                }

                // Regular single-line command
                if !trimmed.is_empty() {
                    let args = tokenize_input(&trimmed);
                    let _ = crate::key_forge::execute_command::execute_command(&args, false);
                }
            }
        }

        // If file ends but buffer still contains something, try to execute it
        if !buffer.trim().is_empty() {
            let args = tokenize_input(buffer.trim());
            let _ = crate::key_forge::execute_command::execute_command(&args, false);
        }
    } else {
        println!("{}", format!("Cannot open file '{}'", filename).red());
    }
}

pub fn interpret_arguments_from_command_line(_args: &[String]) -> Result<(), String> {
    Err("Not implemented".to_string())
}
