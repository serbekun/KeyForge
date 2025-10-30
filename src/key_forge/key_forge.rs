// [file name]: key_forge.rs
// [file content begin]
use colored::Colorize;
use lazy_static::lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, BufRead};
use std::sync::Mutex;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;

use rustyline::Editor;

use rustyline::error::ReadlineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParsedValue {
    Int(i32),
    Float(f64),
    String(String),
    Array(Vec<ParsedValue>),
    Dictionary(HashMap<String, ParsedValue>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variables {
    pub int_variables: HashMap<String, i32>,
    pub float_variables: HashMap<String, f64>,
    pub string_variables: HashMap<String, String>,
    pub array_variables: HashMap<String, Vec<ParsedValue>>,
    pub dict_variables: HashMap<String, HashMap<String, ParsedValue>>,
}

impl Variables {
    pub fn new() -> Self {
        Self {
            int_variables: HashMap::new(),
            float_variables: HashMap::new(),
            string_variables: HashMap::new(),
            array_variables: HashMap::new(),
            dict_variables: HashMap::new(),
        }
    }

    pub fn has_variable(&self, name: &str) -> bool {
        self.int_variables.contains_key(name)
            || self.float_variables.contains_key(name)
            || self.string_variables.contains_key(name)
            || self.array_variables.contains_key(name)
            || self.dict_variables.contains_key(name)
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

    pub fn get_array_data(&self, name: &str) -> Result<Vec<ParsedValue>, String> {
        self.array_variables
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Array variable '{}' not found", name))
    }

    pub fn get_dict_data(&self, name: &str) -> Result<HashMap<String, ParsedValue>, String> {
        self.dict_variables
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Dictionary variable '{}' not found", name))
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

    pub fn add_data_to_array(&mut self, name: String, v: Vec<ParsedValue>) {
        self.array_variables.insert(name, v);
    }

    pub fn add_data_to_dict(&mut self, name: String, v: HashMap<String, ParsedValue>) {
        self.dict_variables.insert(name, v);
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

    #[allow(dead_code)]
    pub fn remove_array_data(&mut self, name: &str) {
        self.array_variables.remove(name);
    }

    #[allow(dead_code)]
    pub fn remove_dict_data(&mut self, name: &str) {
        self.dict_variables.remove(name);
    }

    pub fn remove_string_char(&mut self, name: &str, index: usize) -> Result<(), String> {
        let s = self.get_string_data(name)?;
        let mut chars: Vec<char> = s.chars().collect();
        
        if index >= chars.len() {
            return Err(format!("Index {} out of bounds for string '{}' with length {}", 
                            index, name, chars.len()));
        }
        
        chars.remove(index);
        let new_string = chars.into_iter().collect();
        self.add_data_to_string(name.to_string(), new_string);
        
        Ok(())
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
            "a" => {
                println!("=== Array Variables ===");
                for (k, v) in &self.array_variables {
                    println!("{}: {:?}", k, v);
                }
            }
            "d" => {
                println!("=== Dictionary Variables ===");
                for (k, v) in &self.dict_variables {
                    println!("{}: {:?}", k, v);
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
                println!("");
                println!("=== Array Variables ===");
                for (k, v) in &self.array_variables {
                    println!("{}: {:?}", k, v);
                }
                println!("");
                println!("=== Dictionary Variables ===");
                for (k, v) in &self.dict_variables {
                    println!("{}: {:?}", k, v);
                }
            }
        }
    }
}

// ... rest of the file remains the same until the parse_value function

pub fn parse_value(raw: &str) -> ParsedValue {
    // Try to parse as array: [1, 2, 3]
    if let Some(array_str) = raw.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        let elements: Vec<&str> = array_str.split(',').map(|s| s.trim()).collect();
        let parsed_elements: Vec<ParsedValue> = elements
            .iter()
            .filter(|&&s| !s.is_empty())
            .map(|&s| parse_value(s))
            .collect();
        return ParsedValue::Array(parsed_elements);
    }

    // Try to parse as dictionary: {key: value, key2: value2}
    if let Some(dict_str) = raw.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
        let mut dict = HashMap::new();
        let pairs: Vec<&str> = dict_str.split(',').map(|s| s.trim()).collect();
        
        for pair in pairs {
            if let Some((key, value)) = pair.split_once(':') {
                let key = key.trim().to_string();
                let value = parse_value(value.trim());
                dict.insert(key, value);
            }
        }
        return ParsedValue::Dictionary(dict);
    }

    // Original parsing logic for basic types
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

// Update store_parsed_value to handle arrays and dictionaries
pub fn store_parsed_value(name: String, value: ParsedValue, _source: Option<&str>) -> Result<(), String> {
    let mut store = get_variable_store().lock().map_err(|e| format!("Mutex poisoned: {}", e))?;

    match value {
        ParsedValue::Int(iv) => store.add_data_to_int(name, iv),
        ParsedValue::Float(fv) => store.add_data_to_float(name, fv),
        ParsedValue::String(sv) => store.add_data_to_string(name, sv),
        ParsedValue::Array(arr) => store.add_data_to_array(name, arr),
        ParsedValue::Dictionary(dict) => store.add_data_to_dict(name, dict),
    }

    Ok(())
}

// Update resolve_to_string to handle arrays and dictionaries
pub fn resolve_to_string(value: &str) -> Result<String, String> {
    let key = if value.starts_with('$') { &value[1..] } else { value };

    let store = get_variable_store().lock().unwrap();

    if let Ok(int_val) = store.get_int_data(key) {
        Ok(int_val.to_string())
    } else if let Ok(float_val) = store.get_float_data(key) {
        Ok(float_val.to_string())
    } else if let Ok(string_val) = store.get_string_data(key) {
        Ok(string_val)
    } else if let Ok(array_val) = store.get_array_data(key) {
        // Convert array to string representation
        let elements: Vec<String> = array_val.iter().map(|v| value_to_string(v)).collect();
        Ok(format!("[{}]", elements.join(", ")))
    } else if let Ok(dict_val) = store.get_dict_data(key) {
        // Convert dictionary to string representation
        let pairs: Vec<String> = dict_val.iter()
            .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
            .collect();
        Ok(format!("{{{}}}", pairs.join(", ")))
    } else {
        // Variable doesn't exist - try parsing as literal
        let parsed_value = parse_value(value);
        match parsed_value {
            ParsedValue::Int(i) => Ok(i.to_string()),
            ParsedValue::Float(f) => Ok(f.to_string()),
            ParsedValue::String(s) => Ok(s),
            ParsedValue::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| value_to_string(v)).collect();
                Ok(format!("[{}]", elements.join(", ")))
            }
            ParsedValue::Dictionary(dict) => {
                let pairs: Vec<String> = dict.iter()
                    .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                    .collect();
                Ok(format!("{{{}}}", pairs.join(", ")))
            }
        }
    }
}

// Helper function to convert ParsedValue to string
pub(crate) fn value_to_string(value: &ParsedValue) -> String {
    match value {
        ParsedValue::Int(i) => i.to_string(),
        ParsedValue::Float(f) => f.to_string(),
        ParsedValue::String(s) => format!("\"{}\"", s),
        ParsedValue::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(value_to_string).collect();
            format!("[{}]", elements.join(", "))
        }
        ParsedValue::Dictionary(dict) => {
            let pairs: Vec<String> = dict.iter()
                .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                .collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
}

// Update save_state_to_file and load_state_from_file to handle arrays and dictionaries
pub fn save_state_to_file(filename: &str, store: &Variables) -> Result<(), String> {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(filename)
        .map_err(|e| format!("Failed to create file '{}': {}", filename, e))?;
    
    // Save integer variables
    for (name, value) in &store.int_variables {
        writeln!(file, "int:{}:{}", name, value)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    // Save float variables
    for (name, value) in &store.float_variables {
        writeln!(file, "float:{}:{}", name, value)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    // Save string variables (escape newlines and colons)
    for (name, value) in &store.string_variables {
        let escaped_value = value.replace("\\", "\\\\").replace(":", "\\:").replace("\n", "\\n");
        writeln!(file, "string:{}:{}", name, escaped_value)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    // Save array variables (using JSON serialization)
    for (name, value) in &store.array_variables {
        let json_value = serde_json::to_string(value)
            .map_err(|e| format!("Failed to serialize array '{}': {}", name, e))?;
        let escaped_json = json_value.replace("\\", "\\\\").replace(":", "\\:").replace("\n", "\\n");
        writeln!(file, "array:{}:{}", name, escaped_json)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    // Save dictionary variables (using JSON serialization)
    for (name, value) in &store.dict_variables {
        let json_value = serde_json::to_string(value)
            .map_err(|e| format!("Failed to serialize dict '{}': {}", name, e))?;
        let escaped_json = json_value.replace("\\", "\\\\").replace(":", "\\:").replace("\n", "\\n");
        writeln!(file, "dict:{}:{}", name, escaped_json)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }
    
    Ok(())
}

pub fn load_state_from_file(filename: &str, store: &mut Variables) -> Result<(), String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    let file = File::open(filename)
        .map_err(|e| format!("Failed to open file '{}': {}", filename, e))?;
    
    let reader = BufReader::new(file);
    
    // Clear existing variables before loading
    store.int_variables.clear();
    store.float_variables.clear();
    store.string_variables.clear();
    store.array_variables.clear();
    store.dict_variables.clear();
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| format!("Failed to read line {}: {}", line_num + 1, e))?;
        let line = line.trim();
        
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        
        let parts: Vec<&str> = line.splitn(3, ':').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid format at line {}: expected 'type:name:value'", line_num + 1));
        }
        
        let var_type = parts[0];
        let name = parts[1];
        let value = parts[2];
        
        match var_type {
            "int" => {
                let int_value = value.parse::<i32>()
                    .map_err(|e| format!("Invalid integer value at line {}: {}", line_num + 1, e))?;
                store.add_data_to_int(name.to_string(), int_value);
            }
            "float" => {
                let float_value = value.parse::<f64>()
                    .map_err(|e| format!("Invalid float value at line {}: {}", line_num + 1, e))?;
                store.add_data_to_float(name.to_string(), float_value);
            }
            "string" => {
                let unescaped_value = value.replace("\\n", "\n").replace("\\:", ":").replace("\\\\", "\\");
                store.add_data_to_string(name.to_string(), unescaped_value);
            }
            "array" => {
                let unescaped_value = value.replace("\\n", "\n").replace("\\:", ":").replace("\\\\", "\\");
                let array_value: Vec<ParsedValue> = serde_json::from_str(&unescaped_value)
                    .map_err(|e| format!("Invalid array value at line {}: {}", line_num + 1, e))?;
                store.add_data_to_array(name.to_string(), array_value);
            }
            "dict" => {
                let unescaped_value = value.replace("\\n", "\n").replace("\\:", ":").replace("\\\\", "\\");
                let dict_value: HashMap<String, ParsedValue> = serde_json::from_str(&unescaped_value)
                    .map_err(|e| format!("Invalid dict value at line {}: {}", line_num + 1, e))?;
                store.add_data_to_dict(name.to_string(), dict_value);
            }
            _ => return Err(format!("Unknown variable type '{}' at line {}", var_type, line_num + 1)),
        }
    }
    
    Ok(())
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

// (Duplicate simple parse/store/resolve functions removed — use the array/dict-capable
// implementations earlier in this file.)

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
    println!("{}", "KeyForge CLI mode".green());

    let mut rl = Editor::<()>::new().unwrap_or_else(|e| {
        eprintln!("Error init CLI: {}", e);
        std::process::exit(1);
    });

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(input);
                
                let args = tokenize_input(input);
                if args.is_empty() {
                    continue;
                }
                
                match crate::key_forge::execute_command::execute_command(&args, false) {
                    Ok(_) => continue,
                    Err(e) => println!("{}", format!("Error: {}", e).red()),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C - exit");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D - exit");
                break;
            },
            Err(err) => {
                println!("Input error: {:?}", err);
                break;
            }
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

pub fn resolve_filename(filename_raw: &str) -> Result<String, String> {
    // Handle command substitution: $(command)
    if filename_raw.starts_with("$(") && filename_raw.ends_with(')') {
        let command_content = &filename_raw[2..filename_raw.len()-1];
        let command_args: Vec<String> = tokenize_input(command_content);
        
        match crate::key_forge::execute_command::execute_command(&command_args, true) {
            Ok(result) => Ok(result.trim().to_string()),
            Err(e) => Err(format!("Error executing command: {}", e)),
        }
    } 
    // Handle variable reference: $variable
    else if filename_raw.starts_with('$') && is_valid_identifier(&filename_raw[1..]) {
        let var_name = &filename_raw[1..];
        let store = get_variable_store().lock().unwrap();
        
        if let Ok(string_val) = store.get_string_data(var_name) {
            Ok(string_val)
        } else {
            Err(format!("String variable {} not found", var_name))
        }
    }
    // Direct filename (remove quotes if present)
    else {
        let filename = filename_raw.trim();
        let filename = filename.strip_prefix('"').unwrap_or(filename);
        let filename = filename.strip_suffix('"').unwrap_or(filename);
        let filename = filename.strip_prefix('\'').unwrap_or(filename);
        let filename = filename.strip_suffix('\'').unwrap_or(filename);
        
        Ok(filename.to_string())
    }
}

// (Duplicate simple save/load implementations removed — the earlier
// JSON-supporting save_state_to_file/load_state_from_file are used.)

pub fn encode_base64(input: &str) -> String {
    STANDARD.encode(input.as_bytes())
}

pub fn decode_base64(input: &str) -> Result<String, String> {
    match STANDARD.decode(input) {
        Ok(decoded_bytes) => {
            match String::from_utf8(decoded_bytes) {
                Ok(decoded_string) => Ok(decoded_string),
                Err(_) => Err("invalid UTF-8 data".to_string())
            }
        }
        Err(e) => Err(format!("Error decode Base64: {}", e))
    }
}