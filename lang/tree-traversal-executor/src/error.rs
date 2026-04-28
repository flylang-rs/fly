use flylang_common::Address;
use flylang_diagnostics::{additions::Note, error::DiagnosticsReport};

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub enum InterpreterError {
    NameNotDefined {
        name: String,
        address: Address
    },
    IncompatibleTypesForBinaryOperation {
        op: String,
        lhs_addr: Address,
        rhs_addr: Address,
        lhs_type: String,
        rhs_type: String,
    },
    IncompatibleTypesForUnaryOperation {
        op: String,
        addr: Address,
        ty: String,
    },
    NoPropertyForType {
        typename: String,
        property: String,
        callee_address: Address
    },
    CallError(CallError)
}

impl InterpreterError {
    pub fn try_get_error_loc(&self) -> Option<&Address> {
        match self {
            InterpreterError::NameNotDefined { address, .. } => Some(address),
            InterpreterError::IncompatibleTypesForBinaryOperation { lhs_addr, .. } => Some(lhs_addr),
            InterpreterError::IncompatibleTypesForUnaryOperation { addr, .. } => Some(addr),
            InterpreterError::CallError(call_error) => call_error.try_get_error_loc(),
            InterpreterError::NoPropertyForType { callee_address, .. } => Some(callee_address),
        }
    }
}

/// Errors that happen when preparing to call a function.
#[rustfmt::skip]
#[derive(Debug, Clone)]
pub enum CallError {
    InsufficentArguments {
        callee_address: Address,
        expected_count: usize,
        given_count: usize,
    }
}

impl CallError {
    pub fn try_get_error_loc(&self) -> Option<&Address> {
        match self {
            CallError::InsufficentArguments { callee_address, .. } => Some(callee_address),
        }
    }
}

impl DiagnosticsReport for InterpreterError {
    fn render(&self) -> String {
        let mut result = String::new();

        match self {
            InterpreterError::NameNotDefined { name, address } => {
                flylang_diagnostics::Diagnostics {}.error_ext(
                    &mut result,
                    &format!("Name `{name}` is not defined."),
                    &address,
                    &[Note::new(address.clone(), "here")],
                    &[],
                )
            }
            InterpreterError::IncompatibleTypesForBinaryOperation {
                op,
                lhs_type,
                rhs_type,
                lhs_addr,
                rhs_addr,
            } => flylang_diagnostics::Diagnostics {}.error_ext(
                &mut result,
                &format!("Incompatible types for binary operation `{op}`: `{lhs_type}` and `{rhs_type}`"),
                &lhs_addr,
                &[
                    // Swap them to make RHS point upper than LHS that shows lower.
                    // "string" + 49
                    //            ^^ Has type: integer
                    // ^^^^^^^^ Has type: string
                    Note::new(rhs_addr.clone(), &format!("Has type: `{rhs_type}`")),
                    Note::new(lhs_addr.clone(), &format!("Has type: `{lhs_type}`")),
                ],
                &[],
            ),
            InterpreterError::IncompatibleTypesForUnaryOperation {
                op,
                ty,
                addr,
            } => flylang_diagnostics::Diagnostics {}.error_ext(
                &mut result,
                &format!("Incompatible types for unary operation `{op}`: `{ty}`"),
                &addr,
                &[
                    Note::new(addr.clone(), &format!("Has type: `{ty}`")),
                ],
                &[],
            ),
            InterpreterError::CallError(err) => {
                match err {
                    CallError::InsufficentArguments { callee_address, expected_count, given_count } => {
                        flylang_diagnostics::Diagnostics {}.error_ext(
                            &mut result,
                            &format!(
                                "Insufficent argument for a function call ({expected_count} expected, {given_count} given)"
                            ),
                            &callee_address,
                            &[
                                Note::new(callee_address.clone(), "here"),
                            ],
                            &[],
                        )
                    }
                }
            }
            InterpreterError::NoPropertyForType { typename, property, callee_address } => {
                flylang_diagnostics::Diagnostics {}.error_ext(
                    &mut result,
                    &format!("Proptery `{property}` is not defined for type `{typename}`"),
                    &callee_address,
                    &[
                        Note::new(callee_address.clone(), "here"),
                    ],
                    &[],
                )
            }
        }

        result
    }
}
