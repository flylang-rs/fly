use crate::{object::Value, realm::Realm};

pub mod integers;
pub mod floats;

pub type RustInteropFn = fn(&mut Realm, &[Value]) -> Value;