use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::ast::Value;

type SharedEnv = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
    parent: Option<SharedEnv>,
    pub variables: HashMap<String, Value>,
}

#[allow(dead_code)]
impl Environment {
    pub fn new_global() -> SharedEnv {
        Rc::new(RefCell::new(Environment {
            parent: None,
            variables: HashMap::new()
        }))
    }

    pub fn new_child(parent: SharedEnv) -> SharedEnv {
        Rc::new(RefCell::new(Environment {
            parent: Some(parent),
            variables: HashMap::new()
        }))
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        // 1. Chercher dans le scope courant
        if let Some(val) = self.variables.get(name) {
            return Some(val.clone());
        }

        // 2. Si non trouvé, chercher dans le scope parent
        if let Some(parent_env) = &self.parent {
            return parent_env.borrow().get_variable(name);
        }

        // 3. Non trouvé du tout
        None
    }
}