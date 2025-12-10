use crate::{Value, NativeFn};
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};
use sha2::{Sha256, Digest};

pub fn register(map: &mut HashMap<String, NativeFn>) {
    map.insert("b64_encode".to_string(), b64_encode);
    map.insert("b64_decode".to_string(), b64_decode);
    map.insert("hash_sha256".to_string(), hash_sha256);
}

fn b64_encode(args: Vec<Value>) -> Result<Value, String> {
    let input = args[0].as_str()?;
    let encoded = general_purpose::STANDARD.encode(input);
    Ok(Value::String(encoded))
}

fn b64_decode(args: Vec<Value>) -> Result<Value, String> {
    let input = args[0].as_str()?;
    let decoded_bytes = general_purpose::STANDARD.decode(input).map_err(|e| e.to_string())?;
    let decoded_str = String::from_utf8(decoded_bytes).map_err(|_| "Invalid UTF-8".to_string())?;
    Ok(Value::String(decoded_str))
}

fn hash_sha256(args: Vec<Value>) -> Result<Value, String> {
    let input = args[0].as_str()?;
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Ok(Value::String(format!("{:x}", result)))
}
