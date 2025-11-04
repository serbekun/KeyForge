use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub fn value_to_string(value: &ParsedValue) -> String {
    match value {
        ParsedValue::Int(i) => i.to_string(),
        ParsedValue::Float(f) => f.to_string(),
        ParsedValue::String(s) => s.to_string(),
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