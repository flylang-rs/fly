use std::sync::{Arc, Mutex};

use crate::{function::Function, runtime::RustInteropFn};

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

impl Value {
	pub fn as_arc_string(&self) -> Option<Arc<String>> {
		if let Value::String(s) = self {
			Some(Arc::clone(s))
		} else {
			None
		}
	}
}

#[derive(Debug, Clone)]
pub enum LValue {
    Identifier(String),
    Index { container: Value, index: Value },
    Property { object: Value, name: String },
}
