use flylang_common::Address;
use flylang_diagnostics::{Diagnostics, additions::Note, error::DiagnosticsReport};
use flylang_lexer::token::{Token, TokenValue};

#[derive(Clone, Debug)]
pub enum ParserError {
    UnexpectedEOF(Address),
    UnexpectedToken {
        token: Token,
        expected: Option<TokenValue>
    },
    ParsingNumberFailed {
        number: String,
        address: Address,
    },
    InvalidArgumentKind {
        address: Address,
        domain: InvalidArgumentKindDomain,
    },
    ReservedKeywordUsage {
        address: Address,
        keyword: String
    }
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
            ParserError::UnexpectedToken { token, expected } => {
                let error_string = if let Some(expec) = expected {
                    format!("Unexpected token `{}`, expected `{}`", token.value.repr(), expec.repr())
                } else {
                    "Unexpected token".to_string()
                };

                Diagnostics {}.error_ext(
                    &mut report,
                    &error_string,
                    &token.address,
                    &[Note::new(token.address.clone(), "here")],
                    &[],
                );
            }
            ParserError::ParsingNumberFailed { number, address } => {
                Diagnostics {}.error_ext(
                    &mut report,
                    &format!("Failed to parse a number: {number:?}"),
                    address,
                    &[Note::new(address.clone(), "here")],
                    &[],
                );
            }
            ParserError::InvalidArgumentKind { address, domain } => {
                let note_msg = match domain {
                    InvalidArgumentKindDomain::WholeExpression => {
                        "only identifier and argument list by using arrays supported here"
                    }
                    InvalidArgumentKindDomain::OnlyId => "only identifiers supported here",
                };

                Diagnostics {}.error_ext(
                    &mut report,
                    "Invalid argument kind",
                    address,
                    &[Note::new(address.clone(), note_msg)],
                    &[],
                );
            }
            ParserError::ReservedKeywordUsage { address, keyword } => {
                Diagnostics {}.error_ext(
                    &mut report,
                    &format!("Cannot use reserved keyword `{keyword}` here"),
                    address,
                    &[Note::new(address.clone(), "here")],
                    &[],
                );
            }
        }

        report
    }
}
