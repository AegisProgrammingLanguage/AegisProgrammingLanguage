use crate::{Value, NativeFn};
use std::collections::HashMap;
use chrono::Local;

pub fn register(map: &mut HashMap<String, NativeFn>) {
    map.insert("date_now".to_string(), date_now);
    map.insert("date_format".to_string(), date_format);
}

fn date_now(_: Vec<Value>) -> Result<Value, String> {
    // Retourne le timestamp ISO 8601
    Ok(Value::String(Local::now().to_rfc3339()))
}

fn date_format(args: Vec<Value>) -> Result<Value, String> {
    // args: [format_str] (utilise l'heure actuelle) ou [timestamp_iso, format_str]
    let now = Local::now();
    let fmt = args[0].as_str()?;
    Ok(Value::String(now.format(&fmt).to_string()))
}
