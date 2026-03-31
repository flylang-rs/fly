use std::sync::Arc;

use crate::{SharedRealm, common_operation_binary, control_flow::ControlFlow, object::Value, runtime::RustInteropFn};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("string::operator+string", string_add_string),
    ("string::operator*integer", string_mul_integer),
    ("string::to_string", string_to_string),
];

common_operation_binary!(string_add_string, String, String, String, |x: &String, y: &String| Arc::new(x.clone() + y));
common_operation_binary!(string_mul_integer, String, Integer, String, |x: &String, y: &i128| Arc::new(x.repeat(*y as _)));

fn string_to_string(_realm: SharedRealm, args: &[Value]) -> ControlFlow {
    let Value::String(ref i) = args[0] else {
        panic!("It's not a string, it's {:?}", args[0]);
    };

    ControlFlow::Value(Value::String(Arc::clone(i)))
}