use std::sync::{Arc, RwLock};

use crate::{object::Value, realm::Realm};

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub realm: Arc<RwLock<Realm>>
}

impl Module {
    pub fn method_lookup(&self, name: &str) -> Option<Value> {
        let realm = self.realm.read().unwrap();
        realm.lookup(name)
    }
}

// It will avoid stack overflowing
impl core::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("name", &self.name)
            .field("realm", &"...")
            .finish()
    }
}