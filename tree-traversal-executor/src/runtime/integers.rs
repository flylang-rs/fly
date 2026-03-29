use crate::{object::Value, realm::Realm, runtime::RustInteropFn};

pub static EXPORT: &[(&str, RustInteropFn)] = &[
    ("integer::operator+integer", integers_add),
    ("integer::operator+float", integer_add_float),
    ("integer::operator-integer", integers_sub),
    ("integer::operator-float", integer_sub_float),
    ("integer::operator*integer", integers_mul),
    ("integer::operator*float", integer_mul_float),
    ("integer::operator/-integer", integers_div_rdown),
    ("integer::operator/+integer", integers_div_rup),
    // TODO: Implement `integer /(+-) float operations`
];

pub fn integers_add(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Integer(x) = lhs && let Value::Integer(y) = rhs {
        return Value::Integer(x + y);
    }

    todo!("Make it return `result<integer, error>`")
}

pub fn integer_add_float(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Integer(x) = lhs && let Value::Float(y) = rhs {
        let x = *x as f64;

        return Value::Float(x + y);
    }

    todo!("Make it return `result<integer, error>`")
}

pub fn integers_sub(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Integer(x) = lhs && let Value::Integer(y) = rhs {
        return Value::Integer(x - y);
    }

    todo!("Make it return `result<integer, error>`")
}

pub fn integer_sub_float(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Integer(x) = lhs && let Value::Float(y) = rhs {
        let x = *x as f64;

        return Value::Float(x - y);
    }

    todo!("Make it return `result<integer, error>`")
}


pub fn integers_mul(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Integer(x) = lhs && let Value::Integer(y) = rhs {
        return Value::Integer(x * y);
    }

    todo!("Make it return `result<integer, error>`")
}

pub fn integer_mul_float(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Integer(x) = lhs && let Value::Float(y) = rhs {
        let x = *x as f64;

        return Value::Float(x * y);
    }

    todo!("Make it return `result<integer, error>`")
}

pub fn integers_div_rdown(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Integer(x) = lhs && let Value::Integer(y) = rhs {
        return Value::Integer(x / y);
    }

    todo!("Make it return `result<integer, error>`")
}

pub fn integers_div_rup(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Integer(x) = lhs && let Value::Integer(y) = rhs {
        let remainder = x % y;

        return Value::Integer((x / y) + if remainder != 0 { 1 } else { 0 });
    }

    todo!("Make it return `result<integer, error>`")
}