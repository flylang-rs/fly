use std::sync::RwLock;

use crate::{Interpreter, InterpreterResult, control_flow::ControlFlow, object::{Value, module::Module}, 
realm::{Realm, SharedRealm}, runtime::RustInteropFn};

fn func_to_displayable(
    _interpreter: &mut Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let value = args.first().unwrap();

    if let Value::Function(f) = value {
        Ok(ControlFlow::Value(Value::String(format!("function@{:p}", Gc::as_ptr(f)).into())))
    } else {
        panic!("Expected function, found: {value:?}");
    }
}

use dumpster::sync::Gc;
pub fn init(builtins: &Gc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("func"),
        realm: Gc::new(RwLock::new(Realm::dive(Gc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    bind.values_mut().insert(String::from("to_displayable"), Value::Native(RustInteropFn::new(func_to_displayable)));

    drop(bind);

    Some(mo)
}