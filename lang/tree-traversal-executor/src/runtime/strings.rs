use std::sync::{ RwLock};

use crate::{
    InterpreterResult, SharedRealm, common_operation_binary, control_flow::ControlFlow, object::{Value, module::Module, string::FlyString}, realm::Realm, runtime::RustInteropFn
};

common_operation_binary!(
    string_add_string,
    String,
    String,
    String,
    |x: &FlyString, y: &FlyString| x.clone() + y.clone()
);
common_operation_binary!(
    string_mul_integer,
    String,
    Integer,
    String,
    |x: &FlyString, y: &i128| FlyString::new(x.repeat(*y as _))
);

common_operation_binary!(
    strings_eq,
    String,
    String,
    Bool,
    |x: &FlyString, y: &FlyString| x == y
);
common_operation_binary!(
    strings_neq,
    String,
    String,
    Bool,
    |x: &FlyString, y: &FlyString| x != y
);
common_operation_binary!(
    strings_gt,
    String,
    String,
    Bool,
    |x: &FlyString, y: &FlyString| x > y
);
common_operation_binary!(
    strings_lt,
    String,
    String,
    Bool,
    |x: &FlyString, y: &FlyString| x < y
);
common_operation_binary!(
    strings_gte,
    String,
    String,
    Bool,
    |x: &FlyString, y: &FlyString| x >= y
);
common_operation_binary!(
    strings_lte,
    String,
    String,
    Bool,
    |x: &FlyString, y: &FlyString| x <= y
);

fn string_to_string(
    _interpreter: &mut crate::Interpreter,
    _realm: std::borrow::Cow<'_, SharedRealm>,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let Value::String(ref i) = args[0] else {
        panic!("It's not a string, it's {:?}", args[0]);
    };

    Ok(ControlFlow::Value(Value::String(i.clone())))
}

fn string_to_displayable(
    _interpreter: &mut crate::Interpreter,
    _realm: std::borrow::Cow<'_, SharedRealm>,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let Value::String(ref i) = args[0] else {
        panic!("It's not a string, it's {:?}", args[0]);
    };

    let disp = format!("\"{}\"", i.as_str());

    Ok(ControlFlow::Value(Value::String(disp.into())))
}

use dumpster::sync::Gc;
pub fn init(builtins: &Gc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("string"),
        realm: Gc::new(RwLock::new(Realm::dive(Gc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    // To string
    bind.values_mut().insert(String::from("to_string"), Value::Native(RustInteropFn::new(string_to_string)));
    bind.values_mut().insert(String::from("to_displayable"), Value::Native(RustInteropFn::new(string_to_displayable)));

    // Basic operations
    bind.values_mut().insert(String::from("operator+string"), Value::Native(RustInteropFn::new(string_add_string)));
    bind.values_mut().insert(String::from("operator*integer"), Value::Native(RustInteropFn::new(string_mul_integer)));

    // Comparison
    bind.values_mut().insert(String::from("operator==string"), Value::Native(RustInteropFn::new(strings_eq)));
    bind.values_mut().insert(String::from("operator!=string"), Value::Native(RustInteropFn::new(strings_neq)));
    bind.values_mut().insert(String::from("operator>string"), Value::Native(RustInteropFn::new(strings_gt)));
    bind.values_mut().insert(String::from("operator<string"), Value::Native(RustInteropFn::new(strings_lt)));
    bind.values_mut().insert(String::from("operator>=string"), Value::Native(RustInteropFn::new(strings_gte)));
    bind.values_mut().insert(String::from("operator<=string"), Value::Native(RustInteropFn::new(strings_lte)));

    drop(bind);

    Some(mo)
}
