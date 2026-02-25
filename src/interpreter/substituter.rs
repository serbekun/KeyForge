use crate::context::Context;
use crate::interpreter::execute::Executor;
use crate::interpreter::parse;

pub struct Substituter;

impl Substituter {
    pub fn resolve(input: &str, context: &mut Context, executor: &Executor) -> Result<String, String> {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' {
                if chars.peek() == Some(&'(') {
                    chars.next();
                    let cmd_str = Self::extract_until(&mut chars, ')')?;
                    let tokens = parse(&cmd_str)?;
                    let cmd_result = executor.execute(&tokens, context)?;
                    result.push_str(&cmd_result);
                } else {
                    let var_name = Self::extract_identifier(&mut chars);
                    if var_name.is_empty() {
                        result.push('$');
                    } else {
                        match context.get(&var_name) {
                            Some(value) => result.push_str(&value.to_string_value()),
                            None => return Err(format!("Variable '{}' not found", var_name)),
                        }
                    }
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    fn extract_until(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        end: char,
    ) -> Result<String, String> {
        let mut result = String::new();
        let mut depth = 0;

        while let Some(ch) = chars.next() {
            if ch == '(' {
                depth += 1;
                result.push(ch);
            } else if ch == end && depth == 0 {
                return Ok(result);
            } else {
                if ch == ')' {
                    depth -= 1;
                }
                result.push(ch);
            }
        }

        Err(format!("Unclosed substitution: expected '{}'", end))
    }

    fn extract_identifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut result = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        result
    }
}
