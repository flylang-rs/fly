use crate::{object::Value, realm::Realm};

pub mod integers;
pub mod floats;

pub type RustInteropFn = fn(&mut Realm, &[Value]) -> Value;

#[macro_export]
macro_rules! common_operation {
    ($name:ident, $ty1:ident, $ty2:ident, $res_ty:ident, $conv:expr) => {
        pub fn $name(_realm: &mut Realm, args: &[Value]) -> Value {
            let lhs = &args[0];
            let rhs = &args[1];

            if let Value::$ty1(x) = lhs && let Value::$ty2(y) = rhs {
                return Value::$res_ty(($conv)(x, y));
            }

            todo!("Make it return `result<T, error>`")
        }
    };
}