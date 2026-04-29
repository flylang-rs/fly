use std::sync::{Arc, RwLock};

use crate::{
    InterpreterResult, SharedRealm, common_operation_binary, common_operation_unary, control_flow::ControlFlow, object::{Value, module::Module}, realm::Realm
};

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

pub fn init(builtins: &Arc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("real"),
        realm: Arc::new(RwLock::new(Realm::dive(Arc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    // Basic operations
    bind.values_mut().insert(String::from("operator+real"), Value::Native(reals_add));
    bind.values_mut().insert(String::from("operator+integer"), Value::Native(real_add_integer));
    bind.values_mut().insert(String::from("operator-real"), Value::Native(reals_sub));
    bind.values_mut().insert(String::from("operator-integer"), Value::Native(real_sub_integer));
    bind.values_mut().insert(String::from("operator*real"), Value::Native(reals_mul));
    bind.values_mut().insert(String::from("operator*integer"), Value::Native(real_mul_integer));
    bind.values_mut().insert(String::from("operator/real"), Value::Native(reals_div));
    bind.values_mut().insert(String::from("operator/integer"), Value::Native(real_div_integer));
    bind.values_mut().insert(String::from("operator/-real"), Value::Native(reals_div_rdown));
    bind.values_mut().insert(String::from("operator/-integer"), Value::Native(real_div_integer_rdown));
    bind.values_mut().insert(String::from("operator/+real"), Value::Native(reals_div_rup));
    bind.values_mut().insert(String::from("operator/+integer"), Value::Native(real_div_integer_rup));

    // Comparison
    bind.values_mut().insert(String::from("operator==real"), Value::Native(reals_eq));
    bind.values_mut().insert(String::from("operator!=real"), Value::Native(reals_neq));
    bind.values_mut().insert(String::from("operator>real"), Value::Native(reals_gt));
    bind.values_mut().insert(String::from("operator<real"), Value::Native(reals_lt));
    bind.values_mut().insert(String::from("operator>=real"), Value::Native(reals_gte));
    bind.values_mut().insert(String::from("operator<=real"), Value::Native(reals_lte));

    // To string
    bind.values_mut().insert(String::from("to_string"), Value::Native(real_to_string));
    bind.values_mut().insert(String::from("to_displayable"), Value::Native(real_to_displayable));

    drop(bind);

    Some(mo)
}
