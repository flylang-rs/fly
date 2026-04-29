use std::sync::{Arc, RwLock};

use crate::control_flow::ControlFlow;
use crate::object::module::Module;
use crate::realm::Realm;
use crate::{SharedRealm, Value};

use crate::{InterpreterResult, common_operation_binary, common_operation_unary};

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


pub fn init(builtins: &Arc<RwLock<Realm>>) -> Option<Module> {
    let mo = Module {
        name: String::from("integer"),
        realm: Arc::new(RwLock::new(Realm::dive(Arc::clone(builtins)))),
    };

    let mut bind = mo.realm.write().unwrap();

    // To string
    bind.values_mut().insert(String::from("to_string"), Value::Native(integer_to_string));
    bind.values_mut().insert(String::from("to_displayable"), Value::Native(integer_to_string));

    // Basic operations
    bind.values_mut().insert(String::from("operator+integer"), Value::Native(integers_add));
    bind.values_mut().insert(String::from("operator+float"), Value::Native(integer_add_float));
    bind.values_mut().insert(String::from("operator-integer"), Value::Native(integers_sub));
    bind.values_mut().insert(String::from("operator-float"), Value::Native(integer_sub_float));
    bind.values_mut().insert(String::from("operator*integer"), Value::Native(integers_mul));
    bind.values_mut().insert(String::from("operator*float"), Value::Native(integer_mul_float));
    bind.values_mut().insert(String::from("operator/integer"), Value::Native(integers_div));
    bind.values_mut().insert(String::from("operator/-integer"), Value::Native(integers_div_rdown));
    bind.values_mut().insert(String::from("operator/+integer"), Value::Native(integers_div_rup));
    bind.values_mut().insert(String::from("operator%integer"), Value::Native(integers_mod));

    // Binary operations
    bind.values_mut().insert(String::from("operator&integer"), Value::Native(integers_bit_and));
    bind.values_mut().insert(String::from("operator|integer"), Value::Native(integers_bit_or));
    bind.values_mut().insert(String::from("operator<<integer"), Value::Native(integers_bit_shift_left));
    bind.values_mut().insert(String::from("operator>>integer"), Value::Native(integers_bit_shift_right));

    // Comparison
    bind.values_mut().insert(String::from("operator==integer"), Value::Native(integers_eq));
    bind.values_mut().insert(String::from("operator!=integer"), Value::Native(integers_neq));
    bind.values_mut().insert(String::from("operator>integer"), Value::Native(integers_gt));
    bind.values_mut().insert(String::from("operator<integer"), Value::Native(integers_lt));
    bind.values_mut().insert(String::from("operator>=integer"), Value::Native(integers_gte));
    bind.values_mut().insert(String::from("operator<=integer"), Value::Native(integers_lte));

    // Unary operations
    bind.values_mut().insert(String::from("operator-"), Value::Native(integer_neg));

    drop(bind);

    Some(mo)
}
