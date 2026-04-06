use std::sync::{Arc, Mutex};

use log::debug;

use crate::{
    Interpreter, SharedRealm, control_flow::ControlFlow, object::Value, runtime::RustInteropFn,
    types,
};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("array::push", array_push),
    ("array::len", array_len),
    // To string
    ("array::to_string", array_to_string),
    ("array::to_displayable", array_to_displayable),
];

pub fn array_push(_interp: &Interpreter, _realm: SharedRealm, args: &[Value]) -> ControlFlow {
    let Value::Array(arr) = &args[0] else {
        panic!("Expected array, got: {:?}", args[0])
    };
    let value = args[1].clone();

    arr.lock().unwrap().push(value);

    ControlFlow::Value(Value::Nil)
}

pub fn array_len(_interp: &Interpreter, _realm: SharedRealm, args: &[Value]) -> ControlFlow {
    let Value::Array(arr) = &args[0] else {
        panic!("Expected array")
    };
    ControlFlow::Value(Value::Integer(arr.lock().unwrap().len() as i128))
}

fn render_value(
    interpreter: &Interpreter,
    realm: &SharedRealm,
    val: &Value,
    seen: &mut Vec<*const Mutex<Vec<Value>>>,
) -> String {
    if let Value::Array(arr) = val {
        return render_array(interpreter, realm, arr, seen);
    }

    let ty = types::value_to_internal_type(val).unwrap();
    let method_name = format!("{ty}::to_displayable");
    let method = realm
        .read()
        .unwrap()
        .lookup(&method_name)
        .unwrap_or_else(|| {
            panic!(
                "Method `to_displayable` is not implemented for type: {}",
                ty
            )
        });

    interpreter
        .call_func(Arc::clone(realm), method, &[val.clone()])
        .as_value()
        .and_then(|x| x.as_arc_string())
        .map(|s| s.to_string())
        .unwrap_or_else(|| panic!("Failed getting displayable for `{}`", ty))
}

fn render_array(
    interpreter: &Interpreter,
    realm: &SharedRealm,
    array: &Arc<Mutex<Vec<Value>>>,
    seen: &mut Vec<*const Mutex<Vec<Value>>>,
) -> String {
    let ptr = Arc::as_ptr(array);

    if seen.contains(&ptr) {
        return "[...]".to_string();
    }

    seen.push(ptr);

    let parts: Vec<String> = {  // put that into block so guard will be dropped on its end.
        let guard = array.lock().unwrap();

        guard
            .iter()
            .map(|val| render_value(interpreter, realm, val, seen))
            .collect()
    };

    seen.pop();

    format!("[{}]", parts.join(", "))
}

fn array_to_string(interpreter: &Interpreter, realm: SharedRealm, args: &[Value]) -> ControlFlow {
    let Value::Array(array) = &args[0] else {
        panic!("Expected array, got {:?}", args[0]);
    };

    // It's like a stack - with each recursion push value's address onto it.
    // If it encounters value with the same address, it's a cyclic reference, show the "[...]", and it's done.
    let mut seen = Vec::new();
    let result = render_array(interpreter, &realm, array, &mut seen);

    ControlFlow::Value(Value::String(result.into()))
}

fn array_to_displayable(
    interpreter: &Interpreter,
    realm: SharedRealm,
    args: &[Value],
) -> ControlFlow {
    array_to_string(interpreter, realm, args)
}
