use std::sync::{Arc, Mutex, RwLock};

use crate::runtime::RustInteropFn;

pub mod function;
pub mod lvalue;
pub mod module;
pub mod record;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Integer(i128),
    Real(f64),
    String(Arc<String>),
    Array(Arc<Mutex<Vec<Value>>>),
    Function(Arc<function::Function>),
    Native(RustInteropFn),
    
    // Complex types starting from 0.1.1
    Record(Arc<record::Record>),
    RecordInstance(Arc<RwLock<record::RecordInstance>>),

    Module(Arc<module::Module>),
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

    pub fn as_record_instance(&self) -> Option<Arc<RwLock<record::RecordInstance>>> {
        if let Value::RecordInstance(r) = self {
            Some(Arc::clone(r))
        } else {
            None
        }
    }

    pub fn as_record(&self) -> Option<Arc<record::Record>> {
        if let Value::Record(r) = self {
            Some(Arc::clone(r))
        } else {
            None
        }
    }

    pub fn as_module(&self) -> Option<Arc<module::Module>> {
        if let Value::Module(mo) = self {
            Some(Arc::clone(mo))
        } else {
            None
        }
    }
}