use flylang_common::Address;

#[derive(Debug, Clone)]
pub enum LexerError {
    UnknownCharacter {
        character: char,
        span: Address
    },
    InvalidNumberError {
        span: Address
    },
    InvalidDigitForNumberBase {
        span: Address
    },
    UnexpectedEOF {
        span: Address
    },
    EOF
}
