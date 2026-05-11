use std::{
    borrow::Cow, collections::HashMap, sync::RwLock
};

use dumpster::{Trace, sync::Gc};
use log::debug;

use crate::object::Value;

pub type SharedRealm = Gc<RwLock<Realm>>;

/// Realm (Context or Environment) is a place where runtime objects are stored.
#[derive(Debug, Clone, Trace)]
pub struct Realm {
    values: HashMap<String, Value>,
    pub parent: Option<SharedRealm>,
}

impl Default for Realm {
    fn default() -> Self {
        Self::new()
    }
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
        debug!("Dive called!");

        Self {
            values: HashMap::new(),
            parent: Some(shared_realm),
        }
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

    pub fn lookup_ref(&self, term: &str) -> Option<Cow<'_, Value>> {
        // Search in current Realm.
        if let Some(val) = self.values.get(term) {
            return Some(Cow::Borrowed(val));
        }

        // If not found, try searching in parent Realm.
        Some(Cow::Owned(self.parent.as_ref()?.read().unwrap().lookup_ref(term)?.into_owned()))
    }

    pub fn lookup(&self, term: &str) -> Option<Value> {
        self.lookup_ref(term).map(|x| x.into_owned())
    }

    /// Walks up the realm chain and it rewrites the value of variable
    /// if it encounteres a variable with specified name.
    /// It does nothing if there's no variable with specified name.
    /// Returns a boolean indicated does that variable exist or not.
    pub fn write_existing(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_owned(), value);
            true
        } else if let Some(parent) = &self.parent {
            parent.write().unwrap().write_existing(name, value)
        } else {
            false
        }
    }
}

// ...
