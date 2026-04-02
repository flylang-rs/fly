use std::sync::{Arc, Mutex};

use crate::{SharedRealm, control_flow::ControlFlow, function::Function, runtime::RustInteropFn};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(Arc<String>),
    Array(Arc<Mutex<Vec<Value>>>),
    Function(Arc<Function>),
    Native(RustInteropFn),
}

#[derive(Debug, Clone)]
pub enum LValue {
    Identifier(String),
    Index { container: Value, index: Value },
    Property { object: Value, name: String },
}