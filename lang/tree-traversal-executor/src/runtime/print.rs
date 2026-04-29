use std::sync::{Arc, RwLock};

use crate::{
    Interpreter, InterpreterResult, SharedRealm, control_flow::ControlFlow, object::{Value, module::Module}, realm::Realm, types
};

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

        // eprintln!("{method_name:?}");

        let method = realm
            .read()
            .unwrap()
            .lookup(&ty)
            .and_then(|x| x.as_module())
            .map(|x| x.method_lookup("to_string"))
            .flatten()
            .ok_or_else(|| panic!("Method `to_string` is not implemented for type: {ty}"))?;

        let string_value =
            interpreter.call_func(Arc::clone(&realm), None, &method, &[i.clone()])?;

        let ControlFlow::Value(Value::String(display_value)) = string_value else {
            panic!("Failed `{}` to string conversion!", ty);
        };

        print!("{display_value}");

        if n < len - 1 {
            print!(" ");
        }
    }

    println!();

    Ok(ControlFlow::Value(Value::Nil))
}

pub fn init(builtins: &Arc<RwLock<Realm>>) -> Option<Module> {
    let mut bind = builtins.write().unwrap();

    bind.values_mut().insert(String::from("print"), Value::Native(inner_print));

    drop(bind);

    None
}