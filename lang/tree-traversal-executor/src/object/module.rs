use std::sync::{Arc, RwLock};

use crate::realm::Realm;

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub realm: Arc<RwLock<Realm>>
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