pub fn parse(input: &str) -> Result<Vec<String>, String> {
    shell_words::split(input).map_err(|e| e.to_string())
}
