use crate::{Value, NativeFn};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use regex::Regex;

struct RegexState {
    patterns: HashMap<usize, Regex>,
    next_id: usize
}

lazy_static! {
    static ref RE_STATE: Mutex<RegexState> = Mutex::new(RegexState {
        patterns: HashMap::new(),
        next_id: 1,
    });
}

pub fn register(map: &mut HashMap<String, NativeFn>) {
    map.insert("re_new".to_string(), re_new);
    map.insert("re_match".to_string(), re_match);
    map.insert("re_replace".to_string(), re_replace);
}

fn re_new(args: Vec<Value>) -> Result<Value, String> {
    let pattern = args[0].as_str()?;
    let re = Regex::new(&pattern).map_err(|e| format!("Invalid Regex: {}", e))?;

    let mut state = RE_STATE.lock().unwrap();
    let id = state.next_id;
    state.patterns.insert(id, re);
    state.next_id += 1;

    Ok(Value::Integer(id as i64))
}

fn re_match(args: Vec<Value>) -> Result<Value, String> {
    let id = args[0].as_int()? as usize;
    let text = args[1].as_str()?;

    let state = RE_STATE.lock().unwrap();
    if let Some(re) = state.patterns.get(&id) {
        Ok(Value::Boolean(re.is_match(&text)))
    } else {
        Err("Regex ID not found".into())
    }
}

fn re_replace(args: Vec<Value>) -> Result<Value, String> {
    let id = args[0].as_int()? as usize;
    let text = args[1].as_str()?;
    let replacement = args[2].as_str()?;

    let state = RE_STATE.lock().unwrap();
    if let Some(re) = state.patterns.get(&id) {
        let result = re.replace_all(&text, replacement.as_str());
        Ok(Value::String(result.to_string()))
    } else {
        Err("Regex ID not found".into())
    }
}
