use crate::object::Value;

#[derive(Debug)]
pub enum ControlFlow {
    Nothing,
    Value(Value),        // normal expression result
    Return(Value),       // return statement fired
    Break,               // for loops later
    Continue,            // for loops later
}