use crate::{SharedRealm, Value, common_operation_binary, common_operation_unary, control_flow::ControlFlow, runtime::RustInteropFn};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("bool::operator!", bool_not),
    ("bool::operator&&bool", bool_and),
    ("bool::operator||bool", bool_or),

    // To string
    ("bool::to_string", bool_to_string),
];

common_operation_unary!(bool_not, Bool, Bool, |x: &bool| !x);

common_operation_binary!(bool_and, Bool, Bool, Bool, |x: &bool, y: &bool| *x && *y);
common_operation_binary!(bool_or, Bool, Bool, Bool, |x: &bool, y: &bool| *x || *y);

fn bool_to_string(_realm: SharedRealm, args: &[Value]) -> ControlFlow {
    let Value::Bool(i) = args[0] else {
        panic!("Exptected bool, got {:?}", args[0]);
    };

    ControlFlow::Value(Value::String(i.to_string().into()))
}