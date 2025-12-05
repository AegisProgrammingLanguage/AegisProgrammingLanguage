use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::ast::{ClassDefinition, Value};

type SharedEnv = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
    parent: Option<SharedEnv>,
    variables: HashMap<String, Value>,
    classes: HashMap<String, ClassDefinition>
}

#[allow(dead_code)]
impl Environment {
    pub fn new_global() -> SharedEnv {
        Rc::new(RefCell::new(Environment {
            parent: None,
            variables: HashMap::new(),
            classes: HashMap::new()
        }))
    }

    pub fn new_child(parent: SharedEnv) -> SharedEnv {
        Rc::new(RefCell::new(Environment {
            parent: Some(parent),
            variables: HashMap::new(),
            classes: HashMap::new()
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

    pub fn define_class(&mut self, def: ClassDefinition) {
        self.classes.insert(def.name.clone(), def);
    }

    pub fn get_class(&self, name: &str) -> Option<ClassDefinition> {
        if let Some(cls) = self.classes.get(name) {
            return Some(cls.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().get_class(name);
        }
        None
    }
}