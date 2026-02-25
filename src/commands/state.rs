use crate::interpreter::Command;
use crate::context::Context;
use std::fs;

pub struct SaveStateCommand;

impl Command for SaveStateCommand {
    fn name(&self) -> &str {
        "save-state"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Err("save-state requires a filename argument".to_string());
        }

        let filename = &args[0];
        let json = context.to_json();
        let json_str = serde_json::to_string_pretty(&json)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;

        fs::write(filename, json_str)
            .map_err(|e| format!("Failed to write state file: {}", e))?;

        Ok(format!("State saved to '{}'", filename))
    }
}

pub struct LoadStateCommand;

impl Command for LoadStateCommand {
    fn name(&self) -> &str {
        "load-state"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Err("load-state requires a filename argument".to_string());
        }

        let filename = &args[0];
        let json_str = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read state file: {}", e))?;

        let json: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse state file: {}", e))?;

        context.from_json(&json)?;
        Ok(format!("State loaded from '{}'", filename))
    }
}
