use crate::{SharedRealm, Value, common_operation_unary, control_flow::ControlFlow, runtime::RustInteropFn};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("bool::operator!", bool_not),

    // To string
    ("bool::to_string", bool_to_string),
];

common_operation_unary!(bool_not, Bool, Bool, |x: &bool| !x);

fn bool_to_string(_realm: SharedRealm, args: &[Value]) -> ControlFlow {
    let Value::Bool(i) = args[0] else {
        panic!("Exptected bool, got {:?}", args[0]);
    };

    ControlFlow::Value(Value::String(i.to_string().into()))
}