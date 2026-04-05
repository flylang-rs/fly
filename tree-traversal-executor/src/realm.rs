use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{SharedRealm, object::Value};

/// Realm (Context or Environment) is a place where runtime objects are stored.

#[derive(Debug, Clone)]
pub struct Realm {
    values: HashMap<String, Value>,
    pub parent: Option<Arc<RwLock<Realm>>>,
}

impl Realm {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    /// Enter new level of realm, recursing deeper.
    pub fn dive(shared_realm: SharedRealm) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(shared_realm),
        }

        // let parent = shared_realm.read().unwrap();
        // Self {
        //     values: HashMap::new(),
        //     parent: Some(Arc::clone(&shared_realm)),
        //     module: parent.module.clone(),
        // }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn into_values(self) -> HashMap<String, Value> {
        self.values
    }

    pub fn values(&self) -> &HashMap<String, Value> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self.values
    }

    pub fn lookup(&self, term: &str) -> Option<Value> {
        // Search in current Realm.
        if let Some(val) = self.values.get(term) {
            return Some(val.clone());
        }

        // If not found, try searching in parent Realm.
        self.parent.as_ref()?.try_read().unwrap().lookup(term)
    }
}

// ...
