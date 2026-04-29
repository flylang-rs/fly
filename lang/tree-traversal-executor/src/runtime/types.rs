use std::sync::{Arc, RwLock};

use crate::{Interpreter, InterpreterResult, SharedRealm, control_flow::ControlFlow, object::{Value, module::Module}, realm::Realm};

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

pub fn init(builtins: &Arc<RwLock<Realm>>) -> Option<Module> {
    let mut bind = builtins.write().unwrap();

    bind.values_mut().insert(String::from("typename"), Value::Native(typename));

    drop(bind);

    None
}
