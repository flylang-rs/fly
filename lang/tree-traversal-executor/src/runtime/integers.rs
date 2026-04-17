use crate::control_flow::ControlFlow;
use crate::{SharedRealm, Value, runtime::RustInteropFn};

use crate::{InterpreterResult, common_operation_binary, common_operation_unary};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("integer::operator+integer", integers_add),
    ("integer::operator+float", integer_add_float),
    ("integer::operator-integer", integers_sub),
    ("integer::operator-float", integer_sub_float),
    ("integer::operator*integer", integers_mul),
    ("integer::operator*float", integer_mul_float),
    ("integer::operator/integer", integers_div),
    ("integer::operator/-integer", integers_div_rdown),
    ("integer::operator/+integer", integers_div_rup),
    ("integer::operator%integer", integers_mod),
    // Binary operations
    ("integer::operator&integer", integers_bit_and),
    ("integer::operator|integer", integers_bit_or),
    ("integer::operator<<integer", integers_bit_shift_left),
    ("integer::operator>>integer", integers_bit_shift_right),
    // Comparison
    ("integer::operator==integer", integers_eq),
    ("integer::operator!=integer", integers_neq),
    ("integer::operator>integer", integers_gt),
    ("integer::operator<integer", integers_lt),
    ("integer::operator>=integer", integers_gte),
    ("integer::operator<=integer", integers_lte),
    // Unary operations
    ("integer::operator-", integer_neg),
    // To string
    ("integer::to_string", integer_to_string),
    ("integer::to_displayable", integer_to_displayable),
];

common_operation_binary!(
    integers_add,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| x + y
);
common_operation_binary!(
    integer_add_float,
    Integer,
    Real,
    Real,
    |x: &i128, y: &f64| (*x as f64) + y
);

common_operation_binary!(
    integers_sub,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| x - y
);
common_operation_binary!(
    integer_sub_float,
    Integer,
    Real,
    Real,
    |x: &i128, y: &f64| (*x as f64) - y
);

common_operation_binary!(
    integers_mul,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| x * y
);
common_operation_binary!(
    integer_mul_float,
    Integer,
    Real,
    Real,
    |x: &i128, y: &f64| (*x as f64) * y
);

common_operation_binary!(
    integers_div,
    Integer,
    Integer,
    Real,
    |x: &i128, y: &i128| (*x as f64) / (*y as f64)
);

common_operation_binary!(
    integers_div_rdown,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| x / y
);
common_operation_binary!(
    integers_div_rup,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| {
        let remainder = x % y;

        (x / y) + if remainder != 0 { 1 } else { 0 }
    }
);

common_operation_binary!(
    integers_mod,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| x % y
);

common_operation_binary!(
    integers_bit_and,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| x & y
);
common_operation_binary!(
    integers_bit_or,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| x | y
);
common_operation_binary!(
    integers_bit_shift_left,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| x << y
);
common_operation_binary!(
    integers_bit_shift_right,
    Integer,
    Integer,
    Integer,
    |x: &i128, y: &i128| x >> y
);

common_operation_binary!(integers_eq, Integer, Integer, Bool, |x: &i128, y: &i128| x
    == y);
common_operation_binary!(
    integers_neq,
    Integer,
    Integer,
    Bool,
    |x: &i128, y: &i128| x != y
);
common_operation_binary!(integers_gt, Integer, Integer, Bool, |x: &i128, y: &i128| x
    > y);
common_operation_binary!(integers_lt, Integer, Integer, Bool, |x: &i128, y: &i128| x
    < y);
common_operation_binary!(
    integers_gte,
    Integer,
    Integer,
    Bool,
    |x: &i128, y: &i128| x >= y
);
common_operation_binary!(
    integers_lte,
    Integer,
    Integer,
    Bool,
    |x: &i128, y: &i128| x <= y
);

common_operation_unary!(integer_neg, Integer, Integer, |x: &i128| -x);

fn integer_to_string(
    _interpreter: &mut crate::Interpreter,
    _realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    let Value::Integer(i) = args[0] else {
        panic!("It's not an integer, it's {:?}", args[0]);
    };

    Ok(ControlFlow::Value(Value::String(i.to_string().into())))
}

fn integer_to_displayable(
    interpreter: &mut crate::Interpreter,
    realm: SharedRealm,
    args: &[Value],
) -> InterpreterResult<ControlFlow> {
    integer_to_string(interpreter, realm, args)
}
