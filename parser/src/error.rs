use flylang_common::Address;
use flylang_diagnostics::{Diagnostics, additions::Note, error::DiagnosticsReport};
use flylang_lexer::token::Token;

#[derive(Clone, Debug)]
pub enum ParserError {
    UnexpectedEOF(Address),
    UnexpectedTokenInExpression { token: Token },
}

impl DiagnosticsReport for ParserError {
    fn render(&self) -> String {
        let mut report = String::new();

        match self {
            ParserError::UnexpectedEOF(addr) => {
                Diagnostics {}.error_ext(
                    &mut report,
                    "Unexpected EOF",
                    addr,
                    &[],
                    &[],
                );
            }
            ParserError::UnexpectedTokenInExpression { token } => {
                Diagnostics {}.error_ext(
                    &mut report,
                    "Unexpected token",
                    &token.address,
                    &[Note::new(token.address.clone(), "here")],
                    &[],
                );
            }
        }

        report
    }
}
