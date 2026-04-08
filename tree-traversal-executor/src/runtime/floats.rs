use crate::{
    SharedRealm, common_operation_binary, common_operation_unary, control_flow::ControlFlow,
    object::Value, runtime::RustInteropFn,
};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("float::operator+float", floats_add),
    ("float::operator+integer", float_add_integer),
    ("float::operator-float", floats_sub),
    ("float::operator-integer", float_sub_integer),
    ("float::operator*float", floats_mul),
    ("float::operator*integer", float_mul_integer),
    ("float::operator/float", floats_div),
    ("float::operator/integer", float_div_integer),
    ("float::operator/-float", floats_div_rdown),
    ("float::operator/-integer", float_div_integer_rdown),
    ("float::operator/+float", floats_div_rup),
    ("float::operator/+integer", float_div_integer_rup),
    // Comparison
    ("float::operator==float", floats_eq),
    ("float::operator!=float", floats_neq),
    ("float::operator>float", floats_gt),
    ("float::operator<float", floats_lt),
    ("float::operator>=float", floats_gte),
    ("float::operator<=float", floats_lte),
    // Unary operations
    ("float::operator-", float_neg),
    // To string
    ("float::to_string", float_to_string),
    ("float::to_displayable", float_to_displayable),
];

common_operation_binary!(floats_add, Float, Float, Float, |x: &f64, y: &f64| x + y);
common_operation_binary!(
    float_add_integer,
    Float,
    Integer,
    Float,
    |x: &f64, y: &i128| x + (*y as f64)
);

common_operation_binary!(floats_sub, Float, Float, Float, |x: &f64, y: &f64| x - y);
common_operation_binary!(
    float_sub_integer,
    Float,
    Integer,
    Float,
    |x: &f64, y: &i128| x - (*y as f64)
);

common_operation_binary!(floats_mul, Float, Float, Float, |x: &f64, y: &f64| x * y);
common_operation_binary!(
    float_mul_integer,
    Float,
    Integer,
    Float,
    |x: &f64, y: &i128| x * (*y as f64)
);

common_operation_binary!(floats_div, Float, Float, Float, |x: &f64, y: &f64| x / y);
common_operation_binary!(
    float_div_integer,
    Float,
    Integer,
    Float,
    |x: &f64, y: &i128| x / (*y as f64)
);

common_operation_binary!(
    floats_div_rdown,
    Float,
    Float,
    Float,
    |x: &f64, y: &f64| (x / y).floor() as i64 as _
);
common_operation_binary!(
    float_div_integer_rdown,
    Float,
    Integer,
    Float,
    |x: &f64, y: &i128| (x / (*y as f64)).floor() as i64 as _
);

common_operation_binary!(
    floats_div_rup,
    Float,
    Float,
    Float,
    |x: &f64, y: &f64| (x / y).ceil() as i64 as _
);
common_operation_binary!(
    float_div_integer_rup,
    Float,
    Integer,
    Float,
    |x: &f64, y: &i128| (x / (*y as f64)).ceil() as i64 as _
);

common_operation_binary!(floats_eq, Float, Float, Bool, |x: &f64, y: &f64| x == y);
common_operation_binary!(floats_neq, Float, Float, Bool, |x: &f64, y: &f64| x != y);
common_operation_binary!(floats_gt, Float, Float, Bool, |x: &f64, y: &f64| x > y);
common_operation_binary!(floats_lt, Float, Float, Bool, |x: &f64, y: &f64| x < y);
common_operation_binary!(floats_gte, Float, Float, Bool, |x: &f64, y: &f64| x >= y);
common_operation_binary!(floats_lte, Float, Float, Bool, |x: &f64, y: &f64| x <= y);

common_operation_unary!(float_neg, Float, Float, |x: &f64| -x);

fn float_to_string(
    _interpreter: &mut crate::Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> ControlFlow {
    let Value::Float(i) = args[0] else {
        panic!("It's not a float, it's {:?}", args[0]);
    };

    ControlFlow::Value(Value::String(i.to_string().into()))
}

fn float_to_displayable(
    interpreter: &mut crate::Interpreter,
    realm: SharedRealm,
    args: &[Value],
) -> ControlFlow {
    float_to_string(interpreter, realm, args)
}
