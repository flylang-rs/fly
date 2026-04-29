use std::sync::{Arc, RwLock};

use crate::{
    Interpreter, InterpreterResult, SharedRealm, control_flow::ControlFlow, object::{Value, module::Module}, realm::Realm
};

fn inner_exit(
    _interpreter: &mut Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let code = args.first();

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

pub fn init(builtins: &Arc<RwLock<Realm>>) -> Option<Module> {
    let mut bind = builtins.write().unwrap();

    bind.values_mut().insert(String::from("exit"), Value::Native(inner_exit));

    drop(bind);

    None
}