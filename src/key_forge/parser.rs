use crate::key_forge::variables::ParsedValue;
use std::collections::HashMap;

pub fn parse_value(raw: &str) -> ParsedValue {
    let trimmed = raw.trim();
    println!("DEBUG parse_value: raw='{}', trimmed='{}'", raw, trimmed);

    // Try to parse as array: [1, 2, 3]
    if let Some(array_str) = trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        let elements: Vec<&str> = split_array_elements(array_str);
        let parsed_elements: Vec<ParsedValue> = elements
            .iter()
            .filter(|&&s| !s.is_empty())
            .map(|&s| parse_value(s))
            .collect();
        return ParsedValue::Array(parsed_elements);
    }

    // Try to parse as dictionary: {key: value, key2: value2}
    if let Some(dict_str) = trimmed.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
        let mut dict = HashMap::new();
        let pairs: Vec<&str> = split_dict_pairs(dict_str);
        
        for pair in pairs {
            if let Some((key, value)) = pair.split_once(':') {
                let key = key.trim().to_string();
                let value = parse_value(value.trim());
                dict.insert(key, value);
            }
        }
        return ParsedValue::Dictionary(dict);
    }

    // Handle quoted strings - remove quotes but keep as String
    if let Some(stripped) = trimmed.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return ParsedValue::String(stripped.to_string());
    }
    if let Some(stripped) = trimmed.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        return ParsedValue::String(stripped.to_string());
    }

    // Try integer
    if let Ok(iv) = trimmed.parse::<i32>() {
        return ParsedValue::Int(iv);
    }

    // Try float
    if let Ok(fv) = trimmed.parse::<f64>() {
        return ParsedValue::Float(fv);
    }

    // If we get here, it's an unquoted string - treat as string without quotes
    ParsedValue::String(trimmed.to_string())
}

// Helper function to split array elements considering nested structures
fn split_array_elements(s: &str) -> Vec<&str> {
    let mut elements = Vec::new();
    let mut start = 0;
    let mut depth = 0;
    let mut in_quotes = false;
    let mut quote_char = '\0';

    for (i, c) in s.char_indices() {
        match c {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = c;
            }
            _ if in_quotes && c == quote_char => {
                in_quotes = false;
            }
            '[' | '{' if !in_quotes => depth += 1,
            ']' | '}' if !in_quotes => depth -= 1,
            ',' if !in_quotes && depth == 0 => {
                elements.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    
    if start < s.len() {
        elements.push(&s[start..]);
    }
    
    elements.iter().map(|s| s.trim()).collect()
}

// Helper function to split dictionary pairs considering nested structures
fn split_dict_pairs(s: &str) -> Vec<&str> {
    let mut pairs = Vec::new();
    let mut start = 0;
    let mut depth = 0;
    let mut in_quotes = false;
    let mut quote_char = '\0';

    for (i, c) in s.char_indices() {
        match c {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = c;
            }
            _ if in_quotes && c == quote_char => {
                in_quotes = false;
            }
            '[' | '{' if !in_quotes => depth += 1,
            ']' | '}' if !in_quotes => depth -= 1,
            ',' if !in_quotes && depth == 0 => {
                pairs.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    
    if start < s.len() {
        pairs.push(&s[start..]);
    }
    
    pairs.iter().map(|s| s.trim()).collect()
}