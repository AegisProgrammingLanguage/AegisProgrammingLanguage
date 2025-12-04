mod ast;
mod environment;
mod parser;
mod interpreter;

use ast::{Instruction};
use environment::Environment;
use std::{fs, env};
use serde_json::Value as JsonValue;

fn main() -> Result<(), String> {
    // 1. Lire le chemin du fichier depuis les arguments CLI
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: aegis <file.json>".to_string());
    }
    let filename = &args[1];

    // 2. Lire et parser le JSON
    let content = fs::read_to_string(filename).map_err(|e| format!("Erreur de lecture du fichier: {}", e))?;
    let json_data: JsonValue = serde_json::from_str(&content).map_err(|e| format!("Erreur de parsing JSON: {}", e))?;

    // Assurez-vous que le JSON est un tableau d'instructions
    let raw_instructions = json_data.as_array().ok_or("Le fichier doit contenir un tableau d'instructions")?;

    // 3. Compiler le JSON brut en AST
    let instructions: Vec<Instruction> = raw_instructions
        .iter()
        .map(|json_instr| parser::parse_instruction(json_instr))
        .collect::<Result<Vec<_>, _>>()?;

    // 4. Initialiser l'environnement et l'évaluateur (Ce code est omis pour la simplicité ici)
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
