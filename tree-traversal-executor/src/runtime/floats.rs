use crate::{object::Value, realm::Realm, runtime::RustInteropFn};

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
    ("float::operator/+float", floats_div_rup),
];

pub fn floats_add(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Float(y) = rhs {
        return Value::Float(x + y);
    }

    todo!("Make it return `result<float, error>`")
}

pub fn float_add_integer(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Integer(y) = rhs {
        let y = *y as f64;

        return Value::Float(x + y);
    }

    todo!("Make it return `result<float, error>`")
}

pub fn floats_sub(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Float(y) = rhs {
        return Value::Float(x - y);
    }

    todo!("Make it return `result<float, error>`")
}

pub fn float_sub_integer(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Integer(y) = rhs {
        let y = *y as f64;

        return Value::Float(x - y);
    }

    todo!("Make it return `result<float, error>`")
}

pub fn floats_mul(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Float(y) = rhs {
        return Value::Float(x * y);
    }

    todo!("Make it return `result<float, error>`")
}

pub fn float_mul_integer(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Integer(y) = rhs {
        let y = *y as f64;

        return Value::Float(x * y);
    }

    todo!("Make it return `result<float, error>`")
}

pub fn floats_div(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Float(y) = rhs {
        // FIXME: I don't know is it okay to do that.
        return Value::Float(x / y);
    }

    todo!("Make it return `result<float, error>`")
}

pub fn float_div_integer(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Integer(y) = rhs {
        let y = *y as f64;

        return Value::Float(x / y);
    }

    todo!("Make it return `result<float, error>`")
}


pub fn floats_div_rdown(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Float(y) = rhs {
        // FIXME: I don't know is it okay to do that.
        return Value::Integer(((x / y).floor() as i64) as _);
    }

    todo!("Make it return `result<float, error>`")
}

pub fn floats_div_rup(_realm: &mut Realm, args: &[Value]) -> Value {
    let lhs = &args[0];
    let rhs = &args[1];

    // TODO: Results

    if let Value::Float(x) = lhs && let Value::Float(y) = rhs {
        return Value::Float(((x / y).ceil() as i64) as _);
    }

    todo!("Make it return `result<float, error>`")
}