use std::sync::RwLock;

use crate::{
    Interpreter, InterpreterResult, control_flow::ControlFlow, object::{Value, module::Module}, realm::{Realm, SharedRealm}, runtime::RustInteropFn, types
};

use dumpster::sync::Gc;

fn inner_print(
    interpreter: &mut Interpreter,
    realm: std::borrow::Cow<'_, SharedRealm>,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let len = args.len();

    for (n, i) in args.iter().enumerate() {
        let ty = types::value_to_internal_type(i).unwrap();

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
                    interpreter.call_func(&realm, None, method, std::slice::from_ref(i))?;

                let ControlFlow::Value(Value::String(display_value)) = value else {
                    panic!("Failed `{}` to string conversion!", ty);
                };

                print!("{}", display_value.as_str());

                if n < len - 1 {
                    print!(" ");
                }

                continue;
            } else {
                panic!("Method `to_string` is not implemented for type: {}", ty);
            }
        }

        let ty = types::value_to_internal_type(i).unwrap();

        // eprintln!("{method_name:?}");

        let method = realm
            .read()
            .unwrap()
            .lookup(&ty)
            .and_then(|x| x.as_module()?.method_lookup("to_string"))
            .ok_or_else(|| panic!("Method `to_string` is not implemented for type: {ty}"))?;

        let string_value =
            interpreter.call_func(&realm, None, &method, std::slice::from_ref(i))?;

        let ControlFlow::Value(Value::String(display_value)) = string_value else {
            panic!("Failed `{}` to string conversion!", ty);
        };

        print!("{}", display_value.as_str());

        if n < len - 1 {
            print!(" ");
        }
    }

    println!();

    Ok(ControlFlow::Value(Value::Nil))
}

pub fn init(builtins: &Gc<RwLock<Realm>>) -> Option<Module> {
    let mut bind = builtins.write().unwrap();

    bind.values_mut().insert(String::from("print"), Value::Native(RustInteropFn::new(inner_print)));

    drop(bind);

    None
}