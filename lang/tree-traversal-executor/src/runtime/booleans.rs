use std::sync::{Arc, RwLock};

use crate::{
    InterpreterResult, Value, common_operation_binary, common_operation_unary, control_flow::ControlFlow, object::module::Module, realm::{Realm, SharedRealm}, runtime::RustInteropFn
};

common_operation_unary!(bool_not, Bool, Bool, |x: &bool| !x);

common_operation_binary!(bool_and, Bool, Bool, Bool, |x: &bool, y: &bool| *x && *y);
common_operation_binary!(bool_or, Bool, Bool, Bool, |x: &bool, y: &bool| *x || *y);

common_operation_binary!(bool_eq, Bool, Bool, Bool, |x: &bool, y: &bool| *x == *y);
common_operation_binary!(bool_neq, Bool, Bool, Bool, |x: &bool, y: &bool| *x != *y);

fn bool_to_string(
    _interpreter: &mut crate::Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let Value::Bool(i) = args[0] else {
        panic!("Exptected bool, got {:?}", args[0]);
    };
    
    Ok(ControlFlow::Value(Value::String(i.to_string().into())))
}

use dumpster::sync::Gc;
pub fn init(builtins: &Gc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("bool"),
        realm: Gc::new(RwLock::new(Realm::dive(Gc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    // To string
    bind.values_mut().insert(String::from("to_string"), Value::Native(RustInteropFn::new(bool_to_string)));
    bind.values_mut().insert(String::from("to_displayable"), Value::Native(RustInteropFn::new(bool_to_string)));

    // Basic operations
    bind.values_mut().insert(String::from("operator!"), Value::Native(RustInteropFn::new(bool_not)));
    bind.values_mut().insert(String::from("operator&&bool"), Value::Native(RustInteropFn::new(bool_and)));
    bind.values_mut().insert(String::from("operator||bool"), Value::Native(RustInteropFn::new(bool_or)));
    
    // Comparison
    bind.values_mut().insert(String::from("operator==bool"), Value::Native(RustInteropFn::new(bool_eq)));
    bind.values_mut().insert(String::from("operator!=bool"), Value::Native(RustInteropFn::new(bool_neq)));

    drop(bind);

    Some(mo)
}
