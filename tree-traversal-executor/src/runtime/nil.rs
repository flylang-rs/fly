use crate::{SharedRealm, control_flow::ControlFlow, object::Value, runtime::RustInteropFn};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("nil::to_string", nil_to_string),
    ("nil::to_displayable", nil_to_string),
];

fn nil_to_string(
    _interpreter: &crate::Interpreter,
    _realm: SharedRealm,
    _args: &[Value],
) -> ControlFlow {
    ControlFlow::Value(Value::String("nil".to_owned().into()))
}