use std::str::CharIndices;
use crate::key_forge::execute_command::execute_command;
use crate::key_forge::key_forge::{get_variable_store, ParsedValue};
use crate::key_forge::input_mode::tokenize_input;

#[derive(Debug, Clone)]
pub enum Token {
    Text(String),
    Variable(String),
    Command(String),
    ArrayStart,
    ArrayEnd,
    Comma,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    #[allow(dead_code)]
    input: &'a str,
    chars: CharIndices<'a>,
    peeked: Option<(usize, char)>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices(),
            peeked: None,
        }
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        if self.peeked.is_none() {
            self.peeked = self.chars.next();
        }
        self.peeked
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        if let Some(peeked) = self.peeked.take() {
            Some(peeked)
        } else {
            self.chars.next()
        }
    }

    fn read_quoted_string(&mut self, quote_char: char) -> String {
        let mut result = String::new();
        result.push(quote_char); // include opening quote
        
        while let Some((_, c)) = self.next_char() {
            result.push(c);
            if c == quote_char {
                break;
            }
        }
        
        result
    }

    fn read_until_delimiter(&mut self) -> String {
        let mut result = String::new();
        
        while let Some((_, c)) = self.peek() {
            if matches!(c, '$' | '[' | ']' | ',' | '"' | '\'' | ' ' | '\t' | '\n' | '\r') {
                break;
            }
            result.push(c);
            self.next_char();
        }
        
        result
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some((_i, c)) = self.next_char() {
            match c {
                '[' => return Some(Token::ArrayStart),
                ']' => return Some(Token::ArrayEnd),
                ',' => return Some(Token::Comma),
                '$' => {
                    if let Some((_, '(')) = self.peek() {
                        self.next_char(); // consume '('
                        let mut cmd = String::new();
                        let mut nested = 1;
                        while let Some((_, c)) = self.next_char() {
                            match c {
                                '(' => nested += 1,
                                ')' => {
                                    nested -= 1;
                                    if nested == 0 { break; }
                                }
                                _ => {}
                            }
                            cmd.push(c);
                        }
                        return Some(Token::Command(cmd));
                    } else {
                        let var = self.read_until_delimiter();
                        if !var.is_empty() {
                            return Some(Token::Variable(var));
                        } else {
                            return Some(Token::Text("$".to_string()));
                        }
                    }
                }
                '"' | '\'' => {
                    let quoted_string = self.read_quoted_string(c);
                    return Some(Token::Text(quoted_string));
                }
                ' ' | '\t' | '\n' | '\r' => continue,
                _ => {
                    let mut text = String::new();
                    text.push(c);
                    text.push_str(&self.read_until_delimiter());
                    return Some(Token::Text(text));
                }
            }
        }
        None
    }
}

/// Evaluates an expression that may contain variables ($var), command substitutions ($(cmd)),
/// and array literals ([1, 2, 3]). Returns a ParsedValue result.
pub fn evaluate_expression(expr: &str) -> Result<ParsedValue, String> {
    let mut lexer = Lexer::new(expr);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    parse_tokens(&tokens)
}

fn parse_tokens(tokens: &[Token]) -> Result<ParsedValue, String> {
    // If first token is '[' and last is ']', treat as array
    if tokens.first().map_or(false, |t| matches!(t, Token::ArrayStart)) &&
       tokens.last().map_or(false, |t| matches!(t, Token::ArrayEnd)) {
        return parse_array(&tokens[1..tokens.len()-1]);
    }

    // Otherwise evaluate as a single value
    match tokens.get(0) {
        Some(Token::Variable(var)) => {
            let store = get_variable_store().lock().unwrap();
            if let Ok(val) = store.get_int_data(var) {
                Ok(ParsedValue::Int(val))
            } else if let Ok(val) = store.get_float_data(var) {
                Ok(ParsedValue::Float(val))
            } else if let Ok(val) = store.get_string_data(var) {
                Ok(ParsedValue::String(val))
            } else {
                Err(format!("Variable '{}' not found", var))
            }
        }
        Some(Token::Command(cmd)) => {
            let args = tokenize_input(&cmd);
            let result = execute_command(&args, true)?;
            // Use parse_value from parser.rs
            Ok(crate::key_forge::parser::parse_value(&result))
        }
        Some(Token::Text(text)) => {
            // Use parse_value from parser.rs - this will handle quoted strings correctly
            Ok(crate::key_forge::parser::parse_value(text))
        }
        _ => Err("Invalid expression".to_string())
    }
}

fn parse_array(tokens: &[Token]) -> Result<ParsedValue, String> {
    let mut elements = Vec::new();
    let mut current_tokens = Vec::new();

    for token in tokens {
        match token {
            Token::Comma if !current_tokens.is_empty() => {
                elements.push(parse_tokens(&current_tokens)?);
                current_tokens.clear();
            }
            Token::Comma => continue, // Skip empty elements
            token => current_tokens.push(token.clone()),
        }
    }

    if !current_tokens.is_empty() {
        elements.push(parse_tokens(&current_tokens)?);
    }

    Ok(ParsedValue::Array(elements))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "[$x, $(get_random_num 1 10), 123, \"hello\"]";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        assert!(matches!(tokens[0], Token::ArrayStart));
        assert!(matches!(&tokens[1], Token::Variable(s) if s == "x"));
        assert!(matches!(tokens[2], Token::Comma));
        assert!(matches!(&tokens[3], Token::Command(s) if s == "get_random_num 1 10"));
        assert!(matches!(tokens[4], Token::Comma));
        assert!(matches!(&tokens[5], Token::Text(s) if s == "123"));
        assert!(matches!(tokens[6], Token::Comma));
        assert!(matches!(&tokens[7], Token::Text(s) if s == "\"hello\""));
        assert!(matches!(tokens[8], Token::ArrayEnd));
    }
}