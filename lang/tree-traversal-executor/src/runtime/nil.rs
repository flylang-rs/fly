use crate::{
    InterpreterResult, SharedRealm, control_flow::ControlFlow, object::Value,
    runtime::RustInteropFn,
};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("nil::to_string", nil_to_string),
    ("nil::to_displayable", nil_to_string),
];

fn nil_to_string(
    _interpreter: &mut crate::Interpreter,
    _realm: SharedRealm,
    _args: &[Value],
) -> InterpreterResult<ControlFlow> {
    Ok(ControlFlow::Value(Value::String("nil".to_owned().into())))
}
