use crate::{runtime::RustInteropFn, Realm, Value};

use crate::common_operation;

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("integer::operator+integer", integers_add),
    ("integer::operator+float", integer_add_float),
    ("integer::operator-integer", integers_sub),
    ("integer::operator-float", integer_sub_float),
    ("integer::operator*integer", integers_mul),
    ("integer::operator*float", integer_mul_float),
    ("integer::operator/-integer", integers_div_rdown),
    ("integer::operator/+integer", integers_div_rup),

    // To string
    ("integer::to_string", integer_to_string),
];

common_operation!(integers_add, Integer, Integer, Integer, |x: &i128, y: &i128| x + y);
common_operation!(integer_add_float, Integer, Float, Float, |x: &i128, y: &f64| (*x as f64) + y);

common_operation!(integers_sub, Integer, Integer, Integer, |x: &i128, y: &i128| x - y);
common_operation!(integer_sub_float, Integer, Float, Float, |x: &i128, y: &f64| (*x as f64) - y);

common_operation!(integers_mul, Integer, Integer, Integer, |x: &i128, y: &i128| x * y);
common_operation!(integer_mul_float, Integer, Float, Float, |x: &i128, y: &f64| (*x as f64) * y);

common_operation!(integers_div_rdown, Integer, Integer, Integer, |x: &i128, y: &i128| x / y);
common_operation!(integers_div_rup, Integer, Integer, Integer, |x: &i128, y: &i128| {
    let remainder = x % y;

    (x / y) + if remainder != 0 { 1 } else { 0 }
});

fn integer_to_string(_realm: &mut Realm, args: &[Value]) -> Value {
    let Value::Integer(i) = args[0] else {
        panic!("It's not an integer, it's {:?}", args[0]);
    };

    Value::String(i.to_string().into())
}