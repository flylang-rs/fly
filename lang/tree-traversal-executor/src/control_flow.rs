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
    pub fn as_value(&self) -> Option<&Value> {
        if let ControlFlow::Value(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn into_value(self) -> Option<Value> {
        if let ControlFlow::Value(val) = self {
            Some(val)
        } else {
            None
        }
    }
}
