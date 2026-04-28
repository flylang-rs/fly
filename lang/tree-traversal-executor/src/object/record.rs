use std::{collections::HashMap, sync::{Arc, RwLock}};

use flylang_common::visibility::Visibility;

use crate::{object::Value, realm::Realm};

#[derive(Debug)]
pub struct Record {
    pub name: String,
    pub fields: Vec<RecordField>,
    pub methods: Arc<RwLock<HashMap<String, Value>>>,
    pub definition_realm: Arc<RwLock<Realm>>,
}

#[derive(Debug)]
pub struct RecordField {
    pub name: String,
    pub visibility: Visibility,
}

impl RecordField {
    pub fn global(name: &str) -> Self {
        Self {
            name: name.to_string(),
            visibility: Visibility::Global,
        }
    }

    pub fn local(name: &str) -> Self {
        Self {
            name: name.to_string(),
            visibility: Visibility::Local,
        }
    }
}

#[derive(Debug)]
pub struct RecordInstance {
    pub record: Arc<Record>,
    pub fields: Vec<RecordInstanceField>,
}

impl RecordInstance {
    pub fn lookup(&self, field_name: &str) -> Option<&Value> {
        self.fields
            .iter()
            .find(|x| x.name == field_name)
            .map(|x| &x.value)
    }

    pub fn lookup_mut(&mut self, field_name: &str) -> Option<& mut Value> {
        self.fields
            .iter_mut()
            .find(|x| x.name == field_name)
            .map(|x| &mut x.value)
    }
}

#[derive(Debug)]
pub struct RecordInstanceField {
    pub name: String,
    pub value: Value,
}