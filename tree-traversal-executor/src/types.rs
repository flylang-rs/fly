use crate::object::Value;

pub const TYPE_ARRAY: &str = "array";
pub const TYPE_FLOAT: &str = "float";
pub const TYPE_FUNCTION: &str = "func";
pub const TYPE_INTEGER: &str = "integer";
pub const TYPE_NATIVE: &str = "native";
pub const TYPE_NIL: &str = "nil";
pub const TYPE_STRING: &str = "string";

pub fn value_to_internal_type(val: &Value) -> Option<&str> {
    match val {
        Value::Nil => Some(TYPE_NIL),
        Value::Integer(_) => Some(TYPE_INTEGER),
        Value::Float(_) => Some(TYPE_FLOAT),
        Value::String(_) => Some(TYPE_STRING),
        Value::Array(_) => Some(TYPE_ARRAY),
        Value::Function(_) => Some(TYPE_FUNCTION),
        Value::Native(_) => Some(TYPE_NATIVE),
        // unk => panic!("Cannot convert {unk:?} to internal type."),
    }
}