use crate::interpreter::Command;
use crate::context::Context;
use crate::value::Value;

pub struct SetCommand;

impl Command for SetCommand {
    fn name(&self) -> &str {
        "set"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 3 {
            return Err("set requires 3 arguments: type name value".to_string());
        }

        let type_name = &args[0];
        let var_name = &args[1];
        let value_str = args[2..].join(" ");

        let value = Value::parse(type_name, &value_str)?;
        context.set(var_name.clone(), value);
        Ok(String::new())
    }
}

pub struct VlCommand;

impl Command for VlCommand {
    fn name(&self) -> &str {
        "vl"
    }

    fn execute(&self, _args: &[String], context: &mut Context) -> Result<String, String> {
        let variables = context.list();
        if variables.is_empty() {
            return Ok("No variables defined".to_string());
        }

        let mut output = String::from("Variables:\n");
        for (name, type_name, value) in variables {
            output.push_str(&format!("  {} ({}): {}\n", name, type_name, value));
        }
        Ok(output.trim_end().to_string())
    }
}

pub struct RmCommand;

impl Command for RmCommand {
    fn name(&self) -> &str {
        "rm"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Err("rm requires a variable name".to_string());
        }

        let var_name = &args[0];
        match context.remove(var_name) {
            Some(_) => Ok(format!("Removed variable '{}'", var_name)),
            None => Err(format!("Variable '{}' not found", var_name)),
        }
    }
}
