use crate::{Interpreter, SharedRealm, control_flow::ControlFlow, object::Value};

pub mod arrays;
pub mod booleans;
pub mod exit;
pub mod floats;
pub mod integers;
pub mod nil;
pub mod print;
pub mod strings;

pub type RustInteropFn = fn(interpreter: &mut Interpreter, realm: SharedRealm, args: &[Value]) -> ControlFlow;

#[macro_export]
macro_rules! common_operation_binary {
    ($name:ident, $ty1:ident, $ty2:ident, $res_ty:ident, $conv:expr) => {
        pub fn $name(
            _interpreter: &mut $crate::Interpreter,
            _realm: $crate::SharedRealm,
            args: &[$crate::object::Value],
        ) -> $crate::control_flow::ControlFlow {
            let lhs = &args[0];
            let rhs = &args[1];

            if let $crate::object::Value::$ty1(x) = lhs
                && let $crate::object::Value::$ty2(y) = rhs
            {
                return $crate::control_flow::ControlFlow::Value($crate::object::Value::$res_ty(
                    ($conv)(x, y),
                ));
            }

            todo!("Make it return `result<T, error>`")
        }
    };
}

#[macro_export]
macro_rules! common_operation_unary {
    ($name:ident, $ty:ident, $res_ty:ident, $conv:expr) => {
        pub fn $name(
            _interpreter: &mut $crate::Interpreter,
            _realm: $crate::SharedRealm,
            args: &[$crate::object::Value],
        ) -> $crate::control_flow::ControlFlow {
            let val = &args[0];

            if let $crate::object::Value::$ty(x) = val {
                return $crate::control_flow::ControlFlow::Value($crate::object::Value::$res_ty(
                    ($conv)(x),
                ));
            }

            todo!("Make it return `result<T, error>`")
        }
    };
}
