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

	pub fn as_integer(&self) -> Option<i128> {
		if let Value::Integer(i) = self {
			Some(*i)
		} else {
			None
		}
	}

	pub fn as_float(&self) -> Option<f64> {
		if let Value::Float(f) = self {
			Some(*f)
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
