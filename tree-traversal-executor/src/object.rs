use std::sync::{Arc, Mutex};

use crate::{SharedRealm, function::Function};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Integer(i128),
    Float(f64),
    String(Arc<String>),
    Array(Arc<Mutex<Vec<Value>>>),
    Function(Arc<Function>),
    Native(fn(SharedRealm, &[Value]) -> Value),
}