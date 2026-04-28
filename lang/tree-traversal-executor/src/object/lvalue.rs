use crate::object::Value;

#[derive(Debug, Clone)]
pub enum LValue {
    Identifier(String),
    PrivateIdentifier(String),
    Index { container: Value, index: Value },
    Property { object: Value, name: String },
}