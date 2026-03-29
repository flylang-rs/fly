use std::sync::RwLock;

use crate::{call_func, object::Value, realm::Realm, runtime::RustInteropFn, types};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("print", inner_print),
];

fn inner_print(realm: &mut Realm, args: &[Value]) -> Value {
    let len = args.len();

    for (n, i) in args.iter().enumerate() {
        let ty = types::value_to_internal_type(&i).unwrap();

        let method_name = format!("{ty}::to_string");
        
        let method = realm.lookup(&method_name);
        
        if let Some(method) = method {
            // FIXME: Fix that clone!
            let string_value = call_func(RwLock::new(realm.clone()).into(), method, &[i.clone()]);

            let Some(Value::String(display_value)) = string_value else {
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

    Value::Nil
}