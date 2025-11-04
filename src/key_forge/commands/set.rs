use crate::key_forge::key_forge::store_parsed_value;
use crate::key_forge::key_forge::{setters};

pub fn run(args: &[String], _capture_output: bool) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: set <name> <value> OR set <collection_name> <key/index> <value>".to_string());
    }

    let third = args[2].as_str();

    let is_literal_start = third.starts_with('[')
        || third.starts_with('{')
        || third.starts_with('"')
        || third.starts_with('\'')
        || (third.starts_with("$(") && third.ends_with(')'))
        || third.starts_with("$(");

    if args.len() == 3 || is_literal_start {
        let name = args[1].clone();
        let raw_value = args[2..].join(" ");

        match crate::key_forge::expression::evaluate_expression(&raw_value) {
            Ok(parsed_value) => {
                store_parsed_value(name, parsed_value, None)?;
                Ok(String::new())
            }
            Err(e) => Err(format!("Error evaluating expression: {}", e)),
        }
    } else {
        setters::set_collection_element(&args)
    }
}
