use std::sync::{Arc, Mutex};

use flylang_common::visibility::Visibility;

use crate::{function::Function, runtime::RustInteropFn};

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

    // Complex types starting from 0.1.1
    Record(Arc<Record>),
    RecordInstance(Arc<Mutex<RecordInstance>>)
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
	pub fields: Vec<RecordField>
}

#[derive(Debug)]
pub struct RecordField {
	pub name: String,
	pub visibility: Visibility,
	// pub value: Value
}

#[derive(Debug)]
pub struct RecordInstance {
	pub record: Arc<Record>,
	pub fields: Vec<RecordInstanceField>
}

#[derive(Debug)]
pub struct RecordInstanceField {
	pub name: String,
	pub value: Value
}
