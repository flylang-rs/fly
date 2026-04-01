use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::{SharedRealm, object::Value};

/// Realm (Context or Environment) is a place where runtime objects are stored.

#[derive(Debug, Clone)]
pub struct Realm {
    values: HashMap<String, Value>,
    pub parent: Option<Arc<RwLock<Realm>>>
}

impl Realm {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None
        }
    }

    /// Enter new level of realm, recursing deeper.
    pub fn dive(shared_realm: SharedRealm) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(shared_realm),
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
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
            Some(val.clone())
        } else {
            // If not found, try searching in parent Realm.
            if let Some(parent) = self.parent.as_ref() {
                parent.try_read().unwrap().lookup(term)
            } else {
                None
            }
        }
    }

    // pub fn lookup_write(&mut self, term: &str, value: Value) -> bool {
    //     // Search in parent Realm.
    //     if let Some(parent) = self.parent.as_ref() {
    //         if !parent.try_write().unwrap().lookup_write(term, value.clone()) {
    //             return false;
    //         }
    //     }

    //     // If value exist, 
    //     if self.values.contains_key(term) {
    //         return true;
    //     }

    //     // If an object with that name is not found in parent realms, make a new one in current realm.
    //     self.values.insert(term.to_string(), value);

    //     false
    // }
}

// ...