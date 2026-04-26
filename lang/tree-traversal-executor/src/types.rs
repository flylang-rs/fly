use std::borrow::Cow::{self, Borrowed, Owned};

use crate::object::Value;

pub const TYPE_ARRAY: &str = "array";
pub const TYPE_BOOL: &str = "bool";
pub const TYPE_REAL: &str = "real";
pub const TYPE_FUNCTION: &str = "func";
pub const TYPE_INTEGER: &str = "integer";
pub const TYPE_NATIVE: &str = "native";
pub const TYPE_NIL: &str = "nil";
pub const TYPE_STRING: &str = "string";

pub fn value_to_internal_type(val: &Value) -> Option<Cow<'_, str>> {
    match val {
        Value::Array(_) => Some(Borrowed(TYPE_ARRAY)),
        Value::Bool(_) => Some(Borrowed(TYPE_BOOL)),
        Value::Real(_) => Some(Borrowed(TYPE_REAL)),
        Value::Function(_) => Some(Borrowed(TYPE_FUNCTION)),
        Value::Integer(_) => Some(Borrowed(TYPE_INTEGER)),
        Value::Native(_) => Some(Borrowed(TYPE_NATIVE)),
        Value::Nil => Some(Borrowed(TYPE_NIL)),
        Value::String(_) => Some(Borrowed(TYPE_STRING)),
        Value::Record(rec) => Some(Owned(format!("(record \"{}\")", rec.name))),
        Value::RecordInstance(reci) => {
            let lt = reci.lock().unwrap().record.name.clone();
            
            Some(Owned(lt))
        }
        Value::Module(m) => {
            Some(Owned(format!("(module \"{}\"", m.name)))
        }
        // unk => panic!("Cannot convert {unk:?} to internal type."),
    }
}
