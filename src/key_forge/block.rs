pub fn parse_block_commands(input: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut current = String::new();
    let mut brace_count = 0;
    let mut in_quotes = false;
    let mut quote_char = '\0';
    
    for c in input.chars() {
        if in_quotes {
            current.push(c);
            if c == quote_char {
                in_quotes = false;
            }
        } else {
            match c {
                '"' | '\'' => {
                    in_quotes = true;
                    quote_char = c;
                    current.push(c);
                }
                '{' => {
                    brace_count += 1;
                    current.push(c);
                }
                '}' => {
                    brace_count -= 1;
                    current.push(c);
                }
                ';' if brace_count == 0 => {
                    if !current.trim().is_empty() {
                        commands.push(current.trim().to_string());
                    }
                    current.clear();
                }
                _ => current.push(c),
            }
        }
    }
    
    if !current.trim().is_empty() {
        commands.push(current.trim().to_string());
    }
    
    commands
}