use std::sync::{Arc, RwLock};

use crate::{Interpreter, InterpreterResult, SharedRealm, control_flow::ControlFlow, object::{Value, module::Module}, realm::Realm, runtime::RustInteropFn};

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

pub fn init(builtins: &Arc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("func"),
        realm: Arc::new(RwLock::new(Realm::dive(Arc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    bind.values_mut().insert(String::from("to_displayable"), Value::Native(func_to_displayable));

    drop(bind);

    Some(mo)
}