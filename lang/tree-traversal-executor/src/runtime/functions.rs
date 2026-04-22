use std::sync::Arc;

use crate::{Interpreter, InterpreterResult, SharedRealm, control_flow::ControlFlow, object::Value, runtime::RustInteropFn};

#[rustfmt::skip]
pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("func::to_displayable", func_to_displayable)
];

fn func_to_displayable(
    _interpreter: &mut Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let value = args.first().unwrap();

    if let Value::Function(f) = value {
        Ok(ControlFlow::Value(Value::String(format!("function@{:p}", Arc::as_ptr(f)).into())))
    } else {
        panic!("Expected function, found: {value:?}");
    }
}
