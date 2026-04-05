use std::sync::Arc;

use crate::{
    Interpreter, SharedRealm, control_flow::ControlFlow, object::Value, runtime::RustInteropFn, types
};

pub static EXPORT: &[(&str, RustInteropFn)] = &[("print", inner_print)];

fn inner_print(interpreter: &Interpreter, realm: SharedRealm, args: &[Value]) -> ControlFlow {
    let len = args.len();

    for (n, i) in args.iter().enumerate() {
        let ty = types::value_to_internal_type(&i).unwrap();

        let method_name = format!("{ty}::to_string");

        let method = realm.read().unwrap().lookup(&method_name);

        if let Some(method) = method {
            // FIXME: Fix that clone!
            let string_value = interpreter.call_func(Arc::clone(&realm), method, &[i.clone()]);

            let ControlFlow::Value(Value::String(display_value)) = string_value else {
                panic!("Failed `{}` to string conversion!", ty);
            };

            print!("{display_value}");

            if n < len - 1 {
                print!(" ");
            }
        } else {
            panic!("Method `to_string` is not implemented for type: {}", ty);
        }
    }

    println!();

    ControlFlow::Nothing
}
