use crate::interpreter::Command;
use crate::context::Context;
use crate::value::Value;

pub struct AddCommand;

impl Command for AddCommand {
    fn name(&self) -> &str {
        "add"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("add requires 2 arguments".to_string());
        }

        let val1_str = &args[0];
        let val2_str = &args[1];

        let val1 = resolve_value(val1_str, context)?;
        let val2 = resolve_value(val2_str, context)?;

        // Check if both can be interpreted as numbers
        let is_val1_numeric = matches!(&val1, Value::Int(_) | Value::Float(_))
            || (matches!(&val1, Value::String(s) if is_numeric_string(s)));
        let is_val2_numeric = matches!(&val2, Value::Int(_) | Value::Float(_))
            || (matches!(&val2, Value::String(s) if is_numeric_string(s)));

        if is_val1_numeric && is_val2_numeric {
            let n1 = val1.as_float()?;
            let n2 = val2.as_float()?;
            Ok((n1 + n2).to_string())
        } else {
            let s1 = val1.as_string();
            let s2 = val2.as_string();
            Ok(format!("{}{}", s1, s2))
        }
    }
}

pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "sub"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("sub requires 2 arguments".to_string());
        }

        let val1 = resolve_value(&args[0], context)?;
        let val2 = resolve_value(&args[1], context)?;

        let n1 = val1.as_float()?;
        let n2 = val2.as_float()?;
        Ok((n1 - n2).to_string())
    }
}

pub struct MulCommand;

impl Command for MulCommand {
    fn name(&self) -> &str {
        "mul"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("mul requires 2 arguments".to_string());
        }

        let val1 = resolve_value(&args[0], context)?;
        let val2 = resolve_value(&args[1], context)?;

        let n1 = val1.as_float()?;
        let n2 = val2.as_float()?;
        Ok((n1 * n2).to_string())
    }
}

pub struct DivCommand;

impl Command for DivCommand {
    fn name(&self) -> &str {
        "div"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("div requires 2 arguments".to_string());
        }

        let val1 = resolve_value(&args[0], context)?;
        let val2 = resolve_value(&args[1], context)?;

        let n1 = val1.as_float()?;
        let n2 = val2.as_float()?;

        if n2 == 0.0 {
            return Err("Division by zero".to_string());
        }

        Ok((n1 / n2).to_string())
    }
}

fn resolve_value(s: &str, context: &Context) -> Result<Value, String> {
    if let Some(var_name) = s.strip_prefix('$') {
        context
            .get(var_name)
            .ok_or_else(|| format!("Variable '{}' not found", var_name))
    } else {
        Ok(Value::String(s.to_string()))
    }
}

fn is_numeric_string(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}
