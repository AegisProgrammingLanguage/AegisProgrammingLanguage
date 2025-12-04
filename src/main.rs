mod ast;
mod environment;
mod parser;
mod interpreter;
mod compiler; // <--- NOUVEAU MODULE

use environment::Environment;
use std::{fs, env};
use serde_json::Value as JsonValue;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return Err("Usage: aegis <file.aeg>".to_string()); }
    let filename = &args[1];
    let content = fs::read_to_string(filename).map_err(|e| e.to_string())?;

    // --- LOGIQUE DE DETECTION Aegis / JSON ---
    let json_data: JsonValue = if filename.ends_with(".aeg") {
        println!("Compiling Aegis...");
        compiler::compile(&content)?
    } else {
        serde_json::from_str(&content).map_err(|e| e.to_string())?
    };
    
    // Suite inchangée (Compilation AST -> Exécution)
    let instructions = parser::parse_block(&json_data)?;
    let global_env = Environment::new_global();

    println!("--- Début de l'exécution ---");
    for instr in instructions {
        if let Err(e) = interpreter::execute(&instr, global_env.clone()) {
            eprintln!("Erreur d'exécution : {}", e);
            break;
        }
    }
    println!("--- Fin ---");

    Ok(())
}
