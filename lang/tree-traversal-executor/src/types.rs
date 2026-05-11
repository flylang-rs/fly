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

pub fn value_to_internal_type(val: &Value) -> Cow<'_, str> {
    match val {
        Value::Array(_) => Borrowed(TYPE_ARRAY),
        Value::Bool(_) => Borrowed(TYPE_BOOL),
        Value::Real(_) => Borrowed(TYPE_REAL),
        Value::Function(_) => Borrowed(TYPE_FUNCTION),
        Value::Integer(_) => Borrowed(TYPE_INTEGER),
        Value::Native(_) => Borrowed(TYPE_NATIVE),
        Value::Nil => Borrowed(TYPE_NIL),
        Value::String(_) => Borrowed(TYPE_STRING),
        Value::Record(rec) => Owned(format!("(record \"{}\")", rec.name)),
        Value::RecordInstance(reci) => {
            let lt = reci.read().unwrap().record.name.clone();
            
            Owned(lt)
        }
        Value::Module(m) => {
            Owned(format!("(module \"{}\")", m.name))
        }
    }
}
