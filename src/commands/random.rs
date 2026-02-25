use crate::interpreter::Command;
use crate::context::Context;
use rand::Rng;

pub struct RandomNumCommand;

impl Command for RandomNumCommand {
    fn name(&self) -> &str {
        "random-num"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.len() < 2 {
            return Err("random-num requires 2 arguments: min max".to_string());
        }

        let min_str = &args[0];
        let max_str = &args[1];

        let min = if let Some(var_name) = min_str.strip_prefix('$') {
            context
                .get(var_name)
                .ok_or_else(|| format!("Variable '{}' not found", var_name))?
                .as_float()?
        } else {
            min_str
                .parse::<f64>()
                .map_err(|_| format!("Invalid number: {}", min_str))?
        };

        let max = if let Some(var_name) = max_str.strip_prefix('$') {
            context
                .get(var_name)
                .ok_or_else(|| format!("Variable '{}' not found", var_name))?
                .as_float()?
        } else {
            max_str
                .parse::<f64>()
                .map_err(|_| format!("Invalid number: {}", max_str))?
        };

        if min > max {
            return Err("min must be <= max".to_string());
        }

        let mut rng = rand::thread_rng();
        
        if min.fract() == 0.0 && max.fract() == 0.0 {
            let result = rng.gen_range(min as i64..=max as i64);
            Ok(result.to_string())
        } else {
            let result = rng.gen_range(min..=max);
            Ok(result.to_string())
        }
    }
}

pub struct RandomCharCommand;

impl Command for RandomCharCommand {
    fn name(&self) -> &str {
        "random-char"
    }

    fn execute(&self, _args: &[String], _context: &mut Context) -> Result<String, String> {
        let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..chars.len());
        Ok(chars.chars().nth(idx).unwrap().to_string())
    }
}

pub struct RandomStringCommand;

impl Command for RandomStringCommand {
    fn name(&self) -> &str {
        "random-string"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        if args.is_empty() {
            return Err("random-string requires a length argument".to_string());
        }

        let len = if let Some(var_name) = args[0].strip_prefix('$') {
            context
                .get(var_name)
                .ok_or_else(|| format!("Variable '{}' not found", var_name))?
                .as_int()?
        } else {
            args[0]
                .parse::<i64>()
                .map_err(|_| format!("Invalid number: {}", args[0]))?
        };

        if len < 0 {
            return Err("Length must be non-negative".to_string());
        }

        let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        let result: String = (0..len)
            .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
            .collect();

        Ok(result)
    }
}
