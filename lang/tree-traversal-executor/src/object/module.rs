use std::sync::{RwLock};
use dumpster::{Trace, sync::Gc};

use crate::{object::Value, realm::{Realm, SharedRealm}};

#[derive(Clone, Trace)]
pub struct Module {
    pub name: String,
    pub realm: SharedRealm
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