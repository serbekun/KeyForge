use crate::key_forge::utils::resolve_to_string;

pub fn evaluate_condition(condition: &str) -> Result<bool, String> {
    let tokens: Vec<&str> = condition.split_whitespace().collect();

    if tokens.len() >= 3 && (tokens.contains(&"and") || tokens.contains(&"or")) {
        return evaluate_complex_condition(&tokens);
    }

    if tokens.len() < 3 {
        return Err("Condition must have at least 3 parts".to_string());
    }

    let left = resolve_to_string(tokens[0])?;
    let operator = tokens[1];
    let right = resolve_to_string(&tokens[2..].join(" "))?;

    if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
        match operator {
            "==" | "eq" => Ok((left_num - right_num).abs() < f64::EPSILON),
            "!=" | "ne" => Ok((left_num - right_num).abs() > f64::EPSILON),
            ">" | "gt" => Ok(left_num > right_num),
            "<" | "lt" => Ok(left_num < right_num),
            ">=" | "ge" => Ok(left_num >= right_num),
            "<=" | "le" => Ok(left_num <= right_num),
            _ => Err(format!("Unknown operator: {}", operator)),
        }
    } else {
        match operator {
            "==" | "eq" => Ok(left == right),
            "!=" | "ne" => Ok(left != right),
            ">" | "gt" => Ok(left > right),
            "<" | "lt" => Ok(left < right),
            ">=" | "ge" => Ok(left >= right),
            "<=" | "le" => Ok(left <= right),
            _ => Err(format!("Unknown operator: {}", operator)),
        }
    }
}

fn evaluate_complex_condition(tokens: &[&str]) -> Result<bool, String> {
    let mut result = None;
    let mut current_operator = "and";
    
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "and" || tokens[i] == "or" {
            current_operator = tokens[i];
            i += 1;
            continue;
        }

        let mut condition_end = i;
        while condition_end < tokens.len() && tokens[condition_end] != "and" && tokens[condition_end] != "or" {
            condition_end += 1;
        }
        
        let condition_tokens = &tokens[i..condition_end];
        if condition_tokens.len() < 3 {
            return Err("Invalid condition in complex expression".to_string());
        }
        
        let simple_condition = condition_tokens.join(" ");
        let condition_result = evaluate_condition(&simple_condition)?;
        
        result = match result {
            None => Some(condition_result),
            Some(current) => match current_operator {
                "and" => Some(current && condition_result),
                "or" => Some(current || condition_result),
                _ => Some(condition_result),
            },
        };
        
        i = condition_end;
    }
    
    result.ok_or("No conditions found".to_string())
}