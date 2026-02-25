//! Execution context for managing variables and state.
//!
//! The `Context` struct maintains all variables defined during program execution.
//! It provides methods for setting, getting, and removing variables, as well as
//! serializing/deserializing the state to/from JSON for persistence.

use std::collections::HashMap;
use crate::value::Value;

/// Execution context that stores all program variables.
///
/// The context is a key-value store where each variable name maps to a `Value`.
/// It is passed through the entire execution pipeline and modified by commands
/// that create or update variables.
///
/// # Example
///
/// ```ignore
/// let mut ctx = Context::new();
/// ctx.set("x".to_string(), Value::Int(42));
/// assert_eq!(ctx.get("x"), Some(Value::Int(42)));
/// ```
pub struct Context {
    variables: HashMap<String, Value>,
}

impl Context {
    /// Creates a new empty context.
    ///
    /// # Returns
    ///
    /// A new context with no variables defined.
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    /// Sets a variable in the context.
    ///
    /// If the variable already exists, it is overwritten with the new value.
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name
    /// * `value` - The value to store
    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    /// Gets a variable from the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name to retrieve
    ///
    /// # Returns
    ///
    /// `Some(Value)` if the variable exists, `None` otherwise.
    /// The value is cloned (values are typically small).
    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }

    /// Removes a variable from the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name to remove
    ///
    /// # Returns
    ///
    /// `Some(Value)` if the variable was present, `None` otherwise.
    pub fn remove(&mut self, name: &str) -> Option<Value> {
        self.variables.remove(name)
    }

    /// Lists all variables in the context.
    ///
    /// # Returns
    ///
    /// A vector of tuples `(name, type, value)` sorted alphabetically by name.
    pub fn list(&self) -> Vec<(String, String, String)> {
        let mut items: Vec<_> = self
            .variables
            .iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    value.type_name().to_string(),
                    value.to_string_value(),
                )
            })
            .collect();
        // Sort alphabetically for consistent output
        items.sort_by(|a, b| a.0.cmp(&b.0));
        items
    }

    /// Clears all variables from the context.
    pub fn clear(&mut self) {
        self.variables.clear();
    }

    /// Serializes the context to a JSON value.
    ///
    /// The JSON structure is an object where each key is a variable name
    /// and each value is an object with `"type"` and `"value"` fields.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` representing the context state.
    ///
    /// # Example
    ///
    /// ```ignore
    /// {
    ///   "x": {"type": "int", "value": "42"},
    ///   "name": {"type": "string", "value": "Sergei"}
    /// }
    /// ```
    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for (name, value) in &self.variables {
            map.insert(
                name.clone(),
                serde_json::json!({
                    "type": value.type_name(),
                    "value": value.to_string_value(),
                }),
            );
        }
        serde_json::Value::Object(map)
    }

    /// Deserializes the context from a JSON value.
    ///
    /// Expects the JSON to be an object with variable names as keys,
    /// where each value is an object with `"type"` and `"value"` fields.
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON value to load from
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error describing the parsing failure.
    ///
    /// # Errors
    ///
    /// - If the JSON root is not an object
    /// - If any variable is missing the `"type"` or `"value"` field
    /// - If any value cannot be parsed as its declared type
    pub fn from_json(&mut self, json: &serde_json::Value) -> Result<(), String> {
        if let Some(obj) = json.as_object() {
            for (name, val) in obj {
                if let Some(obj_val) = val.as_object() {
                    let type_name = obj_val
                        .get("type")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| "Missing 'type' field".to_string())?;
                    let value = obj_val
                        .get("value")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| "Missing 'value' field".to_string())?;
                    let parsed = Value::parse(type_name, value)?;
                    self.set(name.clone(), parsed);
                }
            }
            Ok(())
        } else {
            Err("JSON must be an object".to_string())
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
