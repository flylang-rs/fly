use flylang_common::{Address, spanned::Spanned};
use flylang_diagnostics::{additions::Note, error::DiagnosticsReport};

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub enum InterpreterError {
    NameNotDefined {
        name: String,
        address: Address
    },
    IncompatibleTypesForOperation {
        op: String,
        lhs_addr: Address,
        rhs_addr: Address,
        lhs_type: String,
        rhs_type: String,
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
            InterpreterError::IncompatibleTypesForOperation {
                op,
                lhs_type,
                rhs_type,
                lhs_addr,
                rhs_addr,
            } => flylang_diagnostics::Diagnostics {}.error_ext(
                &mut result,
                &format!("Incompatible types for operation `{op}`: `{lhs_type}` and `{rhs_type}`"),
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
        }

        result
    }
}
