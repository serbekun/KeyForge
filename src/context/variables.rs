use std::collections::HashMap;
use crate::value::Value;

pub struct Context {
    variables: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }

    pub fn remove(&mut self, name: &str) -> Option<Value> {
        self.variables.remove(name)
    }

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
        items.sort_by(|a, b| a.0.cmp(&b.0));
        items
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }

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
