use std::sync::{Mutex, RwLock};

use dumpster::{Trace, sync::Gc};

use crate::{object::string::FlyString, runtime::RustInteropFn};

pub mod function;
pub mod lvalue;
pub mod module;
pub mod record;
pub mod string;

#[derive(Debug, Clone, Trace)]
pub enum Value {
    Nil,
    Bool(bool),
    Integer(i128),
    Real(f64),
    String(FlyString),
    Array(Gc<Mutex<Vec<Value>>>),
    Function(Gc<function::Function>),
    Native(RustInteropFn),
    
    // Complex types starting from 0.1.1
    Record(Gc<record::Record>),
    RecordInstance(Gc<RwLock<record::RecordInstance>>),

    Module(Gc<module::Module>),
}

impl Value {
    pub fn as_string(&self) -> Option<FlyString> {
        if let Value::String(s) = self {
            Some(s.clone())
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

    pub fn as_record_instance(&self) -> Option<Gc<RwLock<record::RecordInstance>>> {
        if let Value::RecordInstance(r) = self {
            Some(Gc::clone(r))
        } else {
            None
        }
    }

    pub fn as_record(&self) -> Option<Gc<record::Record>> {
        if let Value::Record(r) = self {
            Some(Gc::clone(r))
        } else {
            None
        }
    }

    pub fn as_module(&self) -> Option<Gc<module::Module>> {
        if let Value::Module(mo) = self {
            Some(Gc::clone(mo))
        } else {
            None
        }
    }
}