use std::{cell::RefCell, collections::HashMap, process::Command, rc::Rc};
use crate::{NativeFn, Value};

pub fn register(map: &mut HashMap<String, NativeFn>) {
    map.insert("proc_exec".to_string(), proc_exec);
}

fn proc_exec(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("Args: command, [args_list]".into());
    }

    let cmd_name = args[0].as_str()?;
    let mut command = Command::new(cmd_name);

    if args.len() > 1 {
        if let Value::List(l) = &args[1] {
            for arg in l.borrow().iter() {
                command.arg(arg.as_str()?);
            }
        }
    }

    let output = command.output().map_err(|e| format!("Exec failed: {}", e))?;
    
    // On retourne un Dict { "code": int, "stdout": string, "stderr": string }
    let mut res_map = HashMap::new();

    let code = output.status.code().unwrap_or(-1) as i64;
    res_map.insert("code".to_string(), Value::Integer(code));

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    res_map.insert("stdout".to_string(), Value::String(stdout));

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    res_map.insert("stderr".to_string(), Value::String(stderr));

    Ok(Value::Dict(Rc::new(RefCell::new(res_map))))
}
