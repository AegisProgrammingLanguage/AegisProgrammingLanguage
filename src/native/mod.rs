use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use crate::ast::environment::NativeFn;

static REGISTRY: OnceLock<RwLock<HashMap<String, NativeFn>>> = OnceLock::new();

pub fn init_registry() {
    let mut map = HashMap::new();

    io::register(&mut map);
    time::register(&mut map);
    random::register(&mut map);
    system::register(&mut map);
    json::register(&mut map);
    http::register(&mut map);
    core::register(&mut map);

    let _ = REGISTRY.set(RwLock::new(map));
}

pub fn find(name: &str) -> Option<NativeFn> {
    let register_lock = REGISTRY.get()?;

    let reader = register_lock.read().ok()?;

    reader.get(name).cloned()
}

pub fn extend_registry(new_funcs: HashMap<String, NativeFn>) {
    if let Some(registry_lock) = REGISTRY.get() {
        if let Ok(mut writer) = registry_lock.write() {
            println!("[Aegis] Chargement de {} nouvelles fonctions natives...", new_funcs.len());

            writer.extend(new_funcs);
        }
        else {
            eprintln!("[Aegis] Erreur : Impossible d'obtenir le verrou d'écriture sur le registre.");
        }
    }
    else {
        eprintln!("[Aegis] Erreur : Registre non initialisé avant le chargement des plugins.");
    }
}

mod io;
mod time;
mod random;
mod system;
mod json;
mod http;
mod core;