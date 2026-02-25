use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Binary(Vec<u8>),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Binary(_) => "binary",
        }
    }

    pub fn to_string_value(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Binary(bytes) => format!("<binary: {} bytes>", bytes.len()),
        }
    }

    pub fn as_int(&self) -> Result<i64, String> {
        match self {
            Value::Int(i) => Ok(*i),
            Value::Float(f) => Ok(*f as i64),
            Value::String(s) => s
                .parse::<i64>()
                .map_err(|_| format!("Cannot convert '{}' to int", s)),
            Value::Bool(b) => Ok(if *b { 1 } else { 0 }),
            Value::Binary(_) => Err("Cannot convert binary to int".to_string()),
        }
    }

    pub fn as_float(&self) -> Result<f64, String> {
        match self {
            Value::Int(i) => Ok(*i as f64),
            Value::Float(f) => Ok(*f),
            Value::String(s) => s
                .parse::<f64>()
                .map_err(|_| format!("Cannot convert '{}' to float", s)),
            Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Value::Binary(_) => Err("Cannot convert binary to float".to_string()),
        }
    }

    pub fn as_string(&self) -> String {
        self.to_string_value()
    }

    pub fn as_bool(&self) -> Result<bool, String> {
        match self {
            Value::Bool(b) => Ok(*b),
            Value::Int(i) => Ok(*i != 0),
            Value::Float(f) => Ok(*f != 0.0),
            Value::String(s) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" => Ok(true),
                "false" | "0" | "no" => Ok(false),
                _ => Err(format!("Cannot convert '{}' to bool", s)),
            },
            Value::Binary(_) => Err("Cannot convert binary to bool".to_string()),
        }
    }

    pub fn as_binary(&self) -> Result<Vec<u8>, String> {
        match self {
            Value::Binary(b) => Ok(b.clone()),
            Value::String(s) => Ok(s.as_bytes().to_vec()),
            _ => Err(format!("Cannot convert {} to binary", self.type_name())),
        }
    }

    pub fn parse(type_name: &str, value: &str) -> Result<Value, String> {
        match type_name {
            "int" => value
                .parse::<i64>()
                .map(Value::Int)
                .map_err(|_| format!("Invalid int: {}", value)),
            "float" => value
                .parse::<f64>()
                .map(Value::Float)
                .map_err(|_| format!("Invalid float: {}", value)),
            "string" => Ok(Value::String(value.to_string())),
            "bool" => match value.to_lowercase().as_str() {
                "true" | "1" | "yes" => Ok(Value::Bool(true)),
                "false" | "0" | "no" => Ok(Value::Bool(false)),
                _ => Err(format!("Invalid bool: {}", value)),
            },
            "binary" => Ok(Value::Binary(value.as_bytes().to_vec())),
            _ => Err(format!("Unknown type: {}", type_name)),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}
