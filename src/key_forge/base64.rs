use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;

pub fn encode_base64(input: &str) -> String {
    STANDARD.encode(input.as_bytes())
}

pub fn decode_base64(input: &str) -> Result<String, String> {
    match STANDARD.decode(input) {
        Ok(decoded_bytes) => {
            match String::from_utf8(decoded_bytes) {
                Ok(decoded_string) => Ok(decoded_string),
                Err(_) => Err("invalid UTF-8 data".to_string())
            }
        }
        Err(e) => Err(format!("Error decode Base64: {}", e))
    }
}