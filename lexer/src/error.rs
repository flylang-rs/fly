use flylang_common::Address;
use flylang_diagnostics::{Diagnostics, additions::Note, error::DiagnosticsReport};

#[derive(Debug, Clone)]
pub enum LexerError {
    UnknownCharacter { character: char, span: Address },
    InvalidNumberError { span: Address },
    InvalidDigitForNumberBase { base: usize, span: Address },
    UnexpectedEOF { span: Address },
    EOF,
}

impl DiagnosticsReport for LexerError {
    fn render(&self) -> String {
        let mut report = String::new();

        match self {
            LexerError::UnknownCharacter { character, span } => {
                Diagnostics {}.error_ext(
                    &mut report,
                    &format!("Unknown character: `{character}`"),
                    span,
                    &[Note::new(span.clone(), "here")],
                    &[],
                );
            }
            LexerError::InvalidNumberError { span } => {
                Diagnostics {}.error_ext(
                    &mut report,
                    "Invalid number",
                    span,
                    &[Note::new(span.clone(), "here")],
                    &[],
                );
            },
            LexerError::InvalidDigitForNumberBase { base, span } => {
                Diagnostics {}.error_ext(
                    &mut report,
                    &format!("Invalid digit for current number base {base}"),
                    span,
                    &[Note::new(span.clone(), "here")],
                    &[],
                );
            },
            LexerError::UnexpectedEOF { span } => {
                Diagnostics {}.error_ext(
                    &mut report,
                    "UnexpectedEOF",
                    span,
                    &[Note::new(span.clone(), "here")],
                    &[],
                );
            },
            LexerError::EOF => unreachable!("Not an error, it's a signal for driver that output is reached the end. This is expected EOF."),
        }

        report
    }
}
