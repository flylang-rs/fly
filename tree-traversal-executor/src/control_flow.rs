use crate::object::Value;

#[derive(Debug, Clone)]
pub enum ControlFlow {
    Nothing,
    Value(Value),  // normal expression result
    Return(Value), // return statement fired
    Break,         // for loops
    Continue,      // for loops
}
