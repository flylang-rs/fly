use crate::{Interpreter, InterpreterResult, SharedRealm, control_flow::ControlFlow, object::Value, runtime::RustInteropFn};

#[rustfmt::skip]
pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("typename", typename)
];

fn typename(
    _interpreter: &mut Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    if args.is_empty() {
        panic!("Expected only one argument, got: {}", args.len());
    }

    let val = args.first().unwrap();

    let ty = crate::types::value_to_internal_type(&val).unwrap();

    Ok(ControlFlow::Value(crate::object::Value::String(ty.to_string().into())))
}
