use flylang_common::Address;
use flylang_diagnostics::{Diagnostics, additions::Note, error::DiagnosticsReport};
use flylang_lexer::token::Token;

#[derive(Clone, Debug)]
pub enum ParserError {
    UnexpectedEOF(Address),
    UnexpectedTokenInExpression { token: Token },
    ParsingNumberFailed { number: String, address: Address },
    InvalidArgumentKind {
        address:Address,
        domain: InvalidArgumentKindDomain
    },
}

#[derive(Clone, Debug)]
pub enum InvalidArgumentKindDomain {
    WholeExpression,
    OnlyId,
}

impl DiagnosticsReport for ParserError {
    fn render(&self) -> String {
        let mut report = String::new();

        match self {
            ParserError::UnexpectedEOF(addr) => {
                Diagnostics {}.error_ext(&mut report, "Unexpected EOF", addr, &[], &[]);
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
            ParserError::ParsingNumberFailed { number, address } => {
                Diagnostics {}.error_ext(
                    &mut report,
                    &format!("Failed to parse a number: {number:?}"),
                    &address,
                    &[Note::new(address.clone(), "here")],
                    &[],
                );
            }
            ParserError::InvalidArgumentKind { address, domain } => {
                let note_msg = match domain {
                    InvalidArgumentKindDomain::WholeExpression => "only identifier and argument list by using arrays supported here",
                    InvalidArgumentKindDomain::OnlyId => "only identifiers supported here",
                };

                Diagnostics {}.error_ext(
                    &mut report,
                    &format!("Invalid argument kind"),
                    &address,
                    &[Note::new(
                        address.clone(),
                        note_msg,
                    )],
                    &[],
                );
            }
        }

        report
    }
}
