use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, Weak};

use flylang_common::visibility::Visibility;

use crate::{function::Function, runtime::RustInteropFn};

use crate::realm::Realm;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Integer(i128),
    Real(f64),
    String(Arc<String>),
    Array(Arc<Mutex<Vec<Value>>>),
    Function(Arc<Function>),
    Native(RustInteropFn),
    Module(Arc<Module>),

    // Complex types starting from 0.1.1
    Record(Arc<Record>),
    RecordInstance(Arc<Mutex<RecordInstance>>),
}

impl Value {
    pub fn as_arc_string(&self) -> Option<Arc<String>> {
        if let Value::String(s) = self {
            Some(Arc::clone(s))
        } else {
            None
        }
    }

    pub fn as_integer(&self) -> Option<i128> {
        if let Value::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_real(&self) -> Option<f64> {
        if let Value::Real(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn as_record_instance(&self) -> Option<Arc<Mutex<RecordInstance>>> {
        if let Value::RecordInstance(r) = self {
            Some(Arc::clone(r))
        } else {
            None
        }
    }

    pub fn as_record(&self) -> Option<Arc<Record>> {
        if let Value::Record(r) = self {
            Some(Arc::clone(r))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum LValue {
    Identifier(String),
    PrivateIdentifier(String),
    Index { container: Value, index: Value },
    Property { object: Value, name: String },
}

#[derive(Debug)]
pub struct Record {
    pub name: String,
    pub fields: Vec<RecordField>,
    pub methods: Arc<RwLock<HashMap<String, Value>>>,
    pub definition_realm: Weak<RwLock<Realm>>,
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

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub realm: Arc<RwLock<Realm>>
}