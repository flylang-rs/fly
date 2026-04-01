use crate::control_flow::ControlFlow;
use crate::{runtime::RustInteropFn, SharedRealm, Value};

use crate::common_operation_binary;

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("integer::operator+integer", integers_add),
    ("integer::operator+float", integer_add_float),
    ("integer::operator-integer", integers_sub),
    ("integer::operator-float", integer_sub_float),
    ("integer::operator*integer", integers_mul),
    ("integer::operator*float", integer_mul_float),
    ("integer::operator/-integer", integers_div_rdown),
    ("integer::operator/+integer", integers_div_rup),
    ("integer::operator%integer", integers_mod),

    // Comparison
    ("integer::operator==integer", integers_eq),
    ("integer::operator>integer", integers_gt),
    ("integer::operator<integer", integers_lt),
    ("integer::operator>=integer", integers_gte),
    ("integer::operator<=integer", integers_lte),

    // To string
    ("integer::to_string", integer_to_string),
];

common_operation_binary!(integers_add, Integer, Integer, Integer, |x: &i128, y: &i128| x + y);
common_operation_binary!(integer_add_float, Integer, Float, Float, |x: &i128, y: &f64| (*x as f64) + y);

common_operation_binary!(integers_sub, Integer, Integer, Integer, |x: &i128, y: &i128| x - y);
common_operation_binary!(integer_sub_float, Integer, Float, Float, |x: &i128, y: &f64| (*x as f64) - y);

common_operation_binary!(integers_mul, Integer, Integer, Integer, |x: &i128, y: &i128| x * y);
common_operation_binary!(integer_mul_float, Integer, Float, Float, |x: &i128, y: &f64| (*x as f64) * y);

common_operation_binary!(integers_div_rdown, Integer, Integer, Integer, |x: &i128, y: &i128| x / y);
common_operation_binary!(integers_div_rup, Integer, Integer, Integer, |x: &i128, y: &i128| {
    let remainder = x % y;

    (x / y) + if remainder != 0 { 1 } else { 0 }
});

common_operation_binary!(integers_mod, Integer, Integer, Integer, |x: &i128, y: &i128| x % y);


common_operation_binary!(integers_eq, Integer, Integer, Bool, |x: &i128, y: &i128| x == y);
common_operation_binary!(integers_gt, Integer, Integer, Bool, |x: &i128, y: &i128| x > y);
common_operation_binary!(integers_lt, Integer, Integer, Bool, |x: &i128, y: &i128| x < y);
common_operation_binary!(integers_gte, Integer, Integer, Bool, |x: &i128, y: &i128| x >= y);
common_operation_binary!(integers_lte, Integer, Integer, Bool, |x: &i128, y: &i128| x <= y);

fn integer_to_string(_realm: SharedRealm, args: &[Value]) -> ControlFlow {
    let Value::Integer(i) = args[0] else {
        panic!("It's not an integer, it's {:?}", args[0]);
    };

    ControlFlow::Value(Value::String(i.to_string().into()))
}
