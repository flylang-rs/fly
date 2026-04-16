use crate::object::Value;

#[derive(Debug, Clone)]
pub enum ControlFlow {
    Nothing,
    Value(Value),  // normal expression result
    Return(Value), // return statement fired
    Break,         // for loops
    Continue,      // for loops
}

impl ControlFlow {
    // Complex values are backed by Arc, making clones actually cheap
    pub fn as_value(&self) -> Option<Value> {
        if let ControlFlow::Value(val) = self {
            Some(val.clone())
        } else {
            None
        }
    }
}
