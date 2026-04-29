use std::sync::{Arc, RwLock};

use crate::{
    InterpreterResult, SharedRealm, control_flow::ControlFlow, object::{Value, module::Module}, realm::Realm
};

fn nil_to_string(
    _interpreter: &mut crate::Interpreter,
    _realm: SharedRealm,
    _args: &[Value],
) -> InterpreterResult<ControlFlow> {
    Ok(ControlFlow::Value(Value::String("nil".to_owned().into())))
}

pub fn init(builtins: &Arc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("nil"),
        realm: Arc::new(RwLock::new(Realm::dive(Arc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    bind.values_mut().insert(String::from("to_string"), Value::Native(nil_to_string));
    bind.values_mut().insert(String::from("to_displayable"), Value::Native(nil_to_string));

    drop(bind);

    Some(mo)
}
