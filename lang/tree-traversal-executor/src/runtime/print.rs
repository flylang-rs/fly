use std::sync::Arc;

use crate::{
    Interpreter, InterpreterResult, SharedRealm, control_flow::ControlFlow, object::Value,
    runtime::RustInteropFn, types,
};

#[rustfmt::skip]
pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("print", inner_print)
];

fn inner_print(
    interpreter: &mut Interpreter,
    realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let len = args.len();

    for (n, i) in args.iter().enumerate() {
        let ty = types::value_to_internal_type(&i).unwrap();

        if let Value::RecordInstance(ri) = i {
            if let Some(method) = ri
                .read()
                .unwrap()
                .record
                .methods
                .read()
                .unwrap()
                .get("to_string")
            {
                let value =
                    interpreter.call_func(Arc::clone(&realm), None, method, &[i.clone()])?;

                let ControlFlow::Value(Value::String(display_value)) = value else {
                    panic!("Failed `{}` to string conversion!", ty);
                };

                print!("{display_value}");

                if n < len - 1 {
                    print!(" ");
                }

                continue;
            } else {
                panic!("Method `to_string` is not implemented for type: {}", ty);
            }
        }

        let ty = types::value_to_internal_type(&i).unwrap();

        let method_name = format!("{ty}::to_string");

        // eprintln!("{method_name:?}");

        let method = realm.read().unwrap().lookup(&method_name);

        if let Some(method) = method {
            let string_value =
                interpreter.call_func(Arc::clone(&realm), None, &method, &[i.clone()])?;

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

    Ok(ControlFlow::Value(Value::Nil))
}
