use std::sync::{Arc, Mutex};

use crate::{function::Function, realm::Realm};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Integer(i128),
    Float(f64),
    String(Arc<String>),
    Array(Arc<Mutex<Vec<Value>>>),
    Function(Arc<Function>),
    Native(fn(&mut Realm, &[Value]) -> Value),
}