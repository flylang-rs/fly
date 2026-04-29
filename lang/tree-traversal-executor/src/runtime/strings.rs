use std::sync::{Arc, RwLock};

use crate::{
    InterpreterResult, SharedRealm, common_operation_binary, control_flow::ControlFlow, object::{Value, module::Module}, realm::Realm
};

common_operation_binary!(
    string_add_string,
    String,
    String,
    String,
    |x: &String, y: &String| Arc::new(x.clone() + y)
);
common_operation_binary!(
    string_mul_integer,
    String,
    Integer,
    String,
    |x: &String, y: &i128| Arc::new(x.repeat(*y as _))
);

common_operation_binary!(
    strings_eq,
    String,
    String,
    Bool,
    |x: &String, y: &String| x == y
);
common_operation_binary!(
    strings_neq,
    String,
    String,
    Bool,
    |x: &String, y: &String| x != y
);
common_operation_binary!(
    strings_gt,
    String,
    String,
    Bool,
    |x: &String, y: &String| x > y
);
common_operation_binary!(
    strings_lt,
    String,
    String,
    Bool,
    |x: &String, y: &String| x < y
);
common_operation_binary!(
    strings_gte,
    String,
    String,
    Bool,
    |x: &String, y: &String| x >= y
);
common_operation_binary!(
    strings_lte,
    String,
    String,
    Bool,
    |x: &String, y: &String| x <= y
);

fn string_to_string(
    _interpreter: &mut crate::Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let Value::String(ref i) = args[0] else {
        panic!("It's not a string, it's {:?}", args[0]);
    };

    Ok(ControlFlow::Value(Value::String(Arc::clone(i))))
}

fn string_to_displayable(
    _interpreter: &mut crate::Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let Value::String(ref i) = args[0] else {
        panic!("It's not a string, it's {:?}", args[0]);
    };

    let disp = format!("\"{i}\"");

    Ok(ControlFlow::Value(Value::String(disp.into())))
}

pub fn init(builtins: &Arc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("string"),
        realm: Arc::new(RwLock::new(Realm::dive(Arc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    // To string
    bind.values_mut().insert(String::from("to_string"), Value::Native(string_to_string));
    bind.values_mut().insert(String::from("to_displayable"), Value::Native(string_to_displayable));

    // Basic operations
    bind.values_mut().insert(String::from("operator+string"), Value::Native(string_add_string));
    bind.values_mut().insert(String::from("operator*integer"), Value::Native(string_mul_integer));

    // Comparison
    bind.values_mut().insert(String::from("operator==string"), Value::Native(strings_eq));
    bind.values_mut().insert(String::from("operator!=string"), Value::Native(strings_neq));
    bind.values_mut().insert(String::from("operator>string"), Value::Native(strings_gt));
    bind.values_mut().insert(String::from("operator<string"), Value::Native(strings_lt));
    bind.values_mut().insert(String::from("operator>=string"), Value::Native(strings_gte));
    bind.values_mut().insert(String::from("operator<=string"), Value::Native(strings_lte));

    drop(bind);

    Some(mo)
}
