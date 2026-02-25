//! Core value type system for KeyForge.
//!
//! This module defines the `Value` enum that represents all data types supported by KeyForge:
//! - `Int`: 64-bit signed integers
//! - `Float`: 64-bit floating-point numbers
//! - `String`: UTF-8 encoded text
//! - `Bool`: Boolean values
//! - `Binary`: Raw binary data
//!
//! The `Value` type provides automatic type conversion and supports serialization
//! for state persistence via serde.

use std::fmt;
use serde::{Deserialize, Serialize};

/// Represents all possible data types in KeyForge.
///
/// Each variant corresponds to a specific data type that can be stored in variables
/// or returned from commands. Values can be converted between types with appropriate
/// semantics (e.g., strings can be parsed as numbers).
///
/// # Variants
///
/// - `Int(i64)` - 64-bit signed integer
/// - `Float(f64)` - 64-bit floating-point number
/// - `String(String)` - UTF-8 encoded text string
/// - `Bool(bool)` - Boolean true/false value
/// - `Binary(Vec<u8>)` - Raw binary data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Binary(Vec<u8>),
}

impl Value {
    /// Returns the type name as a string.
    ///
    /// # Returns
    ///
    /// One of: `"int"`, `"float"`, `"string"`, `"bool"`, `"binary"`
    ///
    /// # Example
    ///
    /// ```ignore
    /// assert_eq!(Value::Int(42).type_name(), "int");
    /// assert_eq!(Value::String("hello".to_string()).type_name(), "string");
    /// ```
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Binary(_) => "binary",
        }
    }

    /// Converts the value to its string representation.
    ///
    /// This is used for output and substitution. For binary data, returns
    /// a description like `<binary: N bytes>` instead of raw bytes.
    ///
    /// # Returns
    ///
    /// A String representation of the value.
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Binary(bytes) => format!("<binary: {} bytes>", bytes.len()),
        }
    }

    /// Converts the value to a 64-bit integer.
    ///
    /// # Conversion Rules
    ///
    /// - `Int`: returns as-is
    /// - `Float`: truncates to i64
    /// - `String`: attempts to parse as integer
    /// - `Bool`: true → 1, false → 0
    /// - `Binary`: returns error
    ///
    /// # Errors
    ///
    /// Returns `Err` if the string cannot be parsed as an integer or for binary values.
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

    /// Converts the value to a 64-bit floating-point number.
    ///
    /// # Conversion Rules
    ///
    /// - `Int`: converts to f64
    /// - `Float`: returns as-is
    /// - `String`: attempts to parse as float
    /// - `Bool`: true → 1.0, false → 0.0
    /// - `Binary`: returns error
    ///
    /// # Errors
    ///
    /// Returns `Err` if the string cannot be parsed as a float or for binary values.
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

    /// Returns the string representation (alias for `to_string_value()`).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let v = Value::String("test".to_string());
    /// assert_eq!(v.as_string(), "test");
    /// ```
    pub fn as_string(&self) -> String {
        self.to_string_value()
    }

    /// Converts the value to a boolean.
    ///
    /// # Conversion Rules
    ///
    /// - `Bool`: returns as-is
    /// - `Int`: 0 → false, non-zero → true
    /// - `Float`: 0.0 → false, non-zero → true
    /// - `String`: parses "true"/"yes"/"1" as true, "false"/"no"/"0" as false
    /// - `Binary`: returns error
    ///
    /// # Errors
    ///
    /// Returns `Err` if string doesn't match recognized boolean values or for binary.
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

    /// Returns the binary representation of the value.
    ///
    /// # Conversion Rules
    ///
    /// - `Binary`: returns as-is
    /// - `String`: returns UTF-8 bytes
    /// - Other types: returns error
    ///
    /// # Errors
    ///
    /// Returns `Err` for int, float, and bool values.
    pub fn as_binary(&self) -> Result<Vec<u8>, String> {
        match self {
            Value::Binary(b) => Ok(b.clone()),
            Value::String(s) => Ok(s.as_bytes().to_vec()),
            _ => Err(format!("Cannot convert {} to binary", self.type_name())),
        }
    }

    /// Parses a string into a typed Value.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The type to parse into: "int", "float", "string", "bool", or "binary"
    /// * `value` - The string value to parse
    ///
    /// # Returns
    ///
    /// A `Value` of the specified type, or an error if parsing fails.
    ///
    /// # Errors
    ///
    /// - If `type_name` is not recognized
    /// - If the string cannot be parsed as the requested type
    ///
    /// # Example
    ///
    /// ```ignore
    /// let v = Value::parse("int", "42")?;
    /// assert_eq!(v, Value::Int(42));
    /// ```
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
