use crate::{Value, NativeFn};
use std::collections::HashMap;
use std::path::Path;

pub fn register(map: &mut HashMap<String, NativeFn>) {
    map.insert("path_join".to_string(), path_join);
    map.insert("path_ext".to_string(), path_ext);
    map.insert("path_exists".to_string(), path_exists);
}

fn path_join(args: Vec<Value>) -> Result<Value, String> {
    let p1 = args[0].as_str()?;
    let p2 = args[1].as_str()?;
    let path = Path::new(&p1).join(p2);
    Ok(Value::String(path.to_string_lossy().to_string()))
}

fn path_ext(args: Vec<Value>) -> Result<Value, String> {
    let p = args[0].as_str()?;
    let path = Path::new(&p);
    match path.extension() {
        Some(os_str) => Ok(Value::String(os_str.to_string_lossy().to_string())),
        None => Ok(Value::String("".to_string()))
    }
}

fn path_exists(args: Vec<Value>) -> Result<Value, String> {
    let p = args[0].as_str()?;
    Ok(Value::Boolean(Path::new(&p).exists()))
}
