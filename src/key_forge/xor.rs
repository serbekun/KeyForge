pub fn xor_encrypt_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

/// Encode plain text to hex using XOR
pub fn encode_to_hex(data: &str, key: &str) -> Result<String, String> {
    let encrypted = xor_encrypt_decrypt(data.as_bytes(), key.as_bytes());
    Ok(encrypted.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(" "))
}

/// Decode hex string to plain text using XOR
pub fn decode_hex(hex_data: &str, key: &str) -> Result<String, String> {
    // Remove any spaces from hex string
    let hex_clean = hex_data.replace(' ', "");
    
    // Convert hex string to bytes
    let bytes: Vec<u8> = (0..hex_clean.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex_clean[i..i + 2], 16)
                .map_err(|e| format!("Invalid hex string: {}", e))
        })
        .collect::<Result<Vec<u8>, String>>()?;
    
    // XOR decrypt
    let decrypted = xor_encrypt_decrypt(&bytes, key.as_bytes());
    
    // Convert to UTF-8 string
    String::from_utf8(decrypted)
        .map_err(|e| format!("Invalid UTF-8 sequence after decryption: {}", e))
}