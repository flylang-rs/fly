use crate::{
    InterpreterResult, SharedRealm, common_operation_binary, common_operation_unary, control_flow::ControlFlow, object::Value, runtime::RustInteropFn
};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("real::operator+real", reals_add),
    ("real::operator+integer", real_add_integer),
    ("real::operator-real", reals_sub),
    ("real::operator-integer", real_sub_integer),
    ("real::operator*real", reals_mul),
    ("real::operator*integer", real_mul_integer),
    ("real::operator/real", reals_div),
    ("real::operator/integer", real_div_integer),
    ("real::operator/-real", reals_div_rdown),
    ("real::operator/-integer", real_div_integer_rdown),
    ("real::operator/+real", reals_div_rup),
    ("real::operator/+integer", real_div_integer_rup),
    // Comparison
    ("real::operator==real", reals_eq),
    ("real::operator!=real", reals_neq),
    ("real::operator>real", reals_gt),
    ("real::operator<real", reals_lt),
    ("real::operator>=real", reals_gte),
    ("real::operator<=real", reals_lte),
    // Unary operations
    ("real::operator-", real_neg),
    // To string
    ("real::to_string", real_to_string),
    ("real::to_displayable", real_to_displayable),
];

common_operation_binary!(reals_add, Real, Real, Real, |x: &f64, y: &f64| x + y);
common_operation_binary!(
    real_add_integer,
    Real,
    Integer,
    Real,
    |x: &f64, y: &i128| x + (*y as f64)
);

common_operation_binary!(reals_sub, Real, Real, Real, |x: &f64, y: &f64| x - y);
common_operation_binary!(
    real_sub_integer,
    Real,
    Integer,
    Real,
    |x: &f64, y: &i128| x - (*y as f64)
);

common_operation_binary!(reals_mul, Real, Real, Real, |x: &f64, y: &f64| x * y);
common_operation_binary!(
    real_mul_integer,
    Real,
    Integer,
    Real,
    |x: &f64, y: &i128| x * (*y as f64)
);

common_operation_binary!(reals_div, Real, Real, Real, |x: &f64, y: &f64| x / y);
common_operation_binary!(
    real_div_integer,
    Real,
    Integer,
    Real,
    |x: &f64, y: &i128| x / (*y as f64)
);

common_operation_binary!(
    reals_div_rdown,
    Real,
    Real,
    Real,
    |x: &f64, y: &f64| (x / y).floor() as i64 as _
);
common_operation_binary!(
    real_div_integer_rdown,
    Real,
    Integer,
    Real,
    |x: &f64, y: &i128| (x / (*y as f64)).floor() as i64 as _
);

common_operation_binary!(
    reals_div_rup,
    Real,
    Real,
    Real,
    |x: &f64, y: &f64| (x / y).ceil() as i64 as _
);
common_operation_binary!(
    real_div_integer_rup,
    Real,
    Integer,
    Real,
    |x: &f64, y: &i128| (x / (*y as f64)).ceil() as i64 as _
);

common_operation_binary!(reals_eq, Real, Real, Bool, |x: &f64, y: &f64| x == y);
common_operation_binary!(reals_neq, Real, Real, Bool, |x: &f64, y: &f64| x != y);
common_operation_binary!(reals_gt, Real, Real, Bool, |x: &f64, y: &f64| x > y);
common_operation_binary!(reals_lt, Real, Real, Bool, |x: &f64, y: &f64| x < y);
common_operation_binary!(reals_gte, Real, Real, Bool, |x: &f64, y: &f64| x >= y);
common_operation_binary!(reals_lte, Real, Real, Bool, |x: &f64, y: &f64| x <= y);

common_operation_unary!(real_neg, Real, Real, |x: &f64| -x);

fn real_to_string(
    _interpreter: &mut crate::Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let Value::Real(i) = args[0] else {
        panic!("It's not a float, it's {:?}", args[0]);
    };

    Ok(ControlFlow::Value(Value::String(i.to_string().into())))
}

fn real_to_displayable(
    interpreter: &mut crate::Interpreter,
    realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    real_to_string(interpreter, realm, args)
}
