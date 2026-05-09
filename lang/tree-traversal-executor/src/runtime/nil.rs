use std::sync::{Arc, RwLock};

use crate::{
    InterpreterResult, control_flow::ControlFlow, object::{Value, module::Module}, realm::Realm, runtime::RustInteropFn
};

use crate::SharedRealm;

fn nil_to_string(
    _interpreter: &mut crate::Interpreter,
    _realm: SharedRealm,
    _args: &[Value],
) -> InterpreterResult<ControlFlow> {
    Ok(ControlFlow::Value(Value::String("nil".to_owned().into())))
}

use dumpster::sync::Gc;

pub fn init(builtins: &Gc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("nil"),
        realm: Gc::new(RwLock::new(Realm::dive(Gc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    bind.values_mut().insert(String::from("to_string"), Value::Native(RustInteropFn::new(nil_to_string)));
    bind.values_mut().insert(String::from("to_displayable"), Value::Native(RustInteropFn::new(nil_to_string)));

    drop(bind);

    Some(mo)
}
