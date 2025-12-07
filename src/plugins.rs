use crate::native;
use libloading::{Library, Symbol};
use std::path::Path;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock}; // <--- Nouveaux imports

// Signature que le plugin devra implémenter
type RegisterPluginFn = unsafe extern "C" fn(&mut HashMap<String, crate::NativeFn>);

static LOADED_LIBS: OnceLock<Mutex<Vec<Library>>> = OnceLock::new();

pub fn load_plugin(path_str: &str) -> Result<(), String> {
    let path = Path::new(path_str);

    if !path.exists() {
        return Err(format!("Plugin introuvable : {}", path_str));
    }

    // On prépare le conteneur global si c'est la première fois
    let libs_mutex = LOADED_LIBS.get_or_init(|| Mutex::new(Vec::new()));

    // Le chargement de DLL reste unsafe (c'est inhérent aux FFI)
    unsafe {
        // 1. Charger la DLL
        let lib = Library::new(path).map_err(|e| format!("Erreur chargement DLL: {}", e))?;

        // 2. Chercher le symbole spécial "_aegis_register"
        let func: Symbol<RegisterPluginFn> = lib.get(b"_aegis_register\0")
            .map_err(|e| format!("Le plugin n'a pas de point d'entrée '_aegis_register': {}", e))?;

        // 3. Récupérer le registre natif actuel
        let mut plugin_funcs = HashMap::new();
        func(&mut plugin_funcs);

        // 4. On fusionne dans le registre global
        native::extend_registry(plugin_funcs);

        // 5. On stocke la lib de manière sécurisée avec le Mutex
        // On verrouille la liste juste le temps d'ajouter la lib
        match libs_mutex.lock() {
            Ok(mut libs) => libs.push(lib),
            Err(e) => return Err(format!("Erreur de verrouillage des plugins: {}", e)),
        }
    }

    Ok(())
}
