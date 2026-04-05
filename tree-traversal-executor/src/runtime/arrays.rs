use std::sync::Arc;

use log::debug;

use crate::{SharedRealm, control_flow::ControlFlow, object::Value, runtime::RustInteropFn, types};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("array::to_string", array_to_string),
    ("array::to_displayable", array_to_displayable)
];

fn array_to_string(
    interpreter: &crate::Interpreter,
    realm: SharedRealm,
    args: &[Value],
) -> ControlFlow {
    debug!("Called with realm: {:#?}", realm.read().unwrap().values());

    let Value::Array(array) = &args[0] else {
        panic!("It's not an array, it's {:?}", args[0]);
    };

    let mut repr = String::from("[");

    let bind = array.lock();
    let container = bind.as_deref().unwrap();

    let length = container.len();

    for (idx, val) in container.iter().enumerate() {
        let ty = types::value_to_internal_type(val).unwrap();
        let method_name = format!("{ty}::to_displayable");

        debug!("Method name: {method_name}");

        let method = realm.read().unwrap().lookup(&method_name);

        if let Some(method) = method {
            let string_value = interpreter.call_func(Arc::clone(&realm), method, &[val.clone()]);

            let ControlFlow::Value(Value::String(display_value)) = string_value else {
                panic!("Failed getting displayable representation for type `{}`!", ty);
            };

            let elem_repr = if idx == length - 1 {
                format!("{}", display_value)
            } else {
                format!("{}, ", display_value)
            };

            repr.push_str(&elem_repr);
        } else {
            panic!("Method `to_displayable` is not implemented for type: {}", ty);
        }
    }

    repr.push(']');

    ControlFlow::Value(Value::String(repr.into()))
}

fn array_to_displayable(
    interpreter: &crate::Interpreter,
    realm: SharedRealm,
    args: &[Value],
) -> ControlFlow {
    array_to_string(interpreter, realm, args)
}