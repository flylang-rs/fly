use std::sync::RwLock;

use dumpster::sync::Gc;

use crate::{InterpreterResult, control_flow::ControlFlow, object::{Value, module::Module}, realm::{Realm, SharedRealm}, runtime::RustInteropFn};

fn system_name(
    _interpreter: &mut crate::Interpreter,
    _realm: std::borrow::Cow<'_, SharedRealm>,
    _args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let sysname = format!("{}", os_info::get().os_type());

    Ok(ControlFlow::Value(Value::String(sysname.into())))
}

pub fn init(builtins: &Gc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("system"),
        realm: Gc::new(RwLock::new(Realm::dive(Gc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    bind.values_mut().insert(String::from("name"), Value::Native(RustInteropFn::new(system_name)));

    drop(bind);

    Some(mo)
}