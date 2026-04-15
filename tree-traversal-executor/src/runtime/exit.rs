use std::sync::Arc;

use crate::{
    Interpreter, InterpreterResult, SharedRealm, control_flow::ControlFlow, object::Value, runtime::RustInteropFn, types
};

#[rustfmt::skip]
pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("exit", inner_exit)
];

fn inner_exit(interpreter: &mut Interpreter, realm: SharedRealm, args: &[Value]) -> InterpreterResult<ControlFlow> {
    let code = args.get(0);

    if code.is_none() {
        std::process::exit(0);
    }

    let code = code.unwrap();

    if let Some(code) = code.as_integer() {
        std::process::exit(code as i32)
    } else {
        panic!("Cannot use value `{code:?}` as exit code.")
    }
}
