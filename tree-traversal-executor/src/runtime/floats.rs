use crate::{common_operation, object::Value, realm::Realm, runtime::RustInteropFn};

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
];

common_operation!(floats_add, Float, Float, Float, |x: &f64, y: &f64| x + y);
common_operation!(float_add_integer, Float, Integer, Float, |x: &f64, y: &i128| x + (*y as f64));

common_operation!(floats_sub, Float, Float, Float, |x: &f64, y: &f64| x - y);
common_operation!(float_sub_integer, Float, Integer, Float, |x: &f64, y: &i128| x - (*y as f64));

common_operation!(floats_mul, Float, Float, Float, |x: &f64, y: &f64| x * y);
common_operation!(float_mul_integer, Float, Integer, Float, |x: &f64, y: &i128| x * (*y as f64));

common_operation!(floats_div, Float, Float, Float, |x: &f64, y: &f64| x / y);
common_operation!(float_div_integer, Float, Integer, Float, |x: &f64, y: &i128| x / (*y as f64));

common_operation!(floats_div_rdown, Float, Float, Float, |x: &f64, y: &f64| (x / y).floor() as i64 as _);
common_operation!(float_div_integer_rdown, Float, Integer, Float, |x: &f64, y: &i128| (x / (*y as f64)).floor() as i64 as _);

common_operation!(floats_div_rup, Float, Float, Float, |x: &f64, y: &f64| (x / y).ceil() as i64 as _);
common_operation!(float_div_integer_rup, Float, Integer, Float, |x: &f64, y: &i128| (x / (*y as f64)).ceil() as i64 as _);