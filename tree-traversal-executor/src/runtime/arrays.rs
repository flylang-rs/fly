use std::sync::Arc;

use log::debug;

use crate::{
    control_flow::ControlFlow, object::Value, runtime::RustInteropFn, types, Interpreter,
    SharedRealm,
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

fn array_to_string(interpreter: &Interpreter, realm: SharedRealm, args: &[Value]) -> ControlFlow {
    debug!("Called with realm: {:#?}", realm.read().unwrap().values());

    let Value::Array(array) = &args[0] else {
        panic!("It's not an array, it's {:?}", args[0]);
    };

    let mut repr = String::from("[");

    let bind = array.lock();
    let container = bind.as_deref().unwrap();

    let length = container.len();

    for (idx, val) in container.iter().enumerate() {
        // An important moment: if we have an array in array
        if let Value::Array(arr) = val {
            // And that array point to source array
            if Arc::ptr_eq(arr, array) {
                // It's a circiular reference, it will infinitely scan itself.
                // To avoid this, replace it with ellipsis like Python does.

                let elem_repr = if idx == length - 1 {
                    "[...]"
                } else {
                    "[...], "
                };

                repr.push_str(&elem_repr);
                continue;
            } else {
                type ArrayValue = Arc<std::sync::Mutex<Vec<Value>>>;

                fn check_array(arr1: &ArrayValue, arr2: ArrayValue) -> bool {
                    for i in &*arr2.lock().unwrap() {
                        if let Value::Array(a) = i {
                            if Arc::ptr_eq(&arr1, a) {
                                return true;
                            }

                            if check_array(arr1, Arc::clone(a)) {
                                return true;
                            }
                        }
                    }

                    false
                }

				// TODO: Check, make tests for that case and fix.
                if check_array(&array, Arc::clone(arr)) {
                    let elem_repr = if idx == length - 1 {
                        "[...]"
                    } else {
                        "[...], "
                    };

                    repr.push_str(&elem_repr);
                    continue;
                }
            }
        }

        let ty = types::value_to_internal_type(val).unwrap();
        let method_name = format!("{ty}::to_displayable");

        debug!("Method name: {method_name}");

        let method = realm.read().unwrap().lookup(&method_name);

        if let Some(method) = method {
            let string_value = interpreter.call_func(Arc::clone(&realm), method, &[val.clone()]);

            let ControlFlow::Value(Value::String(display_value)) = string_value else {
                panic!(
                    "Failed getting displayable representation for type `{}`!",
                    ty
                );
            };

            let elem_repr = if idx == length - 1 {
                format!("{}", display_value)
            } else {
                format!("{}, ", display_value)
            };

            repr.push_str(&elem_repr);
        } else {
            panic!(
                "Method `to_displayable` is not implemented for type: {}",
                ty
            );
        }
    }

    repr.push(']');

    ControlFlow::Value(Value::String(repr.into()))
}

fn array_to_displayable(
    interpreter: &Interpreter,
    realm: SharedRealm,
    args: &[Value],
) -> ControlFlow {
    array_to_string(interpreter, realm, args)
}
