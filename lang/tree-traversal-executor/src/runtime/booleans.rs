use std::sync::{Arc, RwLock};

use crate::{
    InterpreterResult, SharedRealm, Value, common_operation_binary, common_operation_unary, control_flow::ControlFlow, object::Module, realm::Realm, runtime::RustInteropFn
};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("bool::operator!", bool_not),
    ("bool::operator&&bool", bool_and),
    ("bool::operator||bool", bool_or),
    // Comparison
    ("bool::operator==bool", bool_eq),
    ("bool::operator!=bool", bool_neq),
    // To string
    // ("bool::to_string", bool_to_string),
    // ("bool::to_displayable", bool_to_displayable),
];

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

pub fn init(builtins: &Arc<RwLock<Realm>>) -> Module {
    let mo = Module {
        name: String::from("bool"),
        realm: Arc::new(RwLock::new(Realm::dive(Arc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    bind.values_mut().insert(String::from("to_string"), Value::Native(bool_to_string));
    bind.values_mut().insert(String::from("to_displayable"), Value::Native(bool_to_string));

    drop(bind);

    mo
}
