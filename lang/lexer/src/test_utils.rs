use core::ops::Range;

use flylang_common::source::Source;

use crate::{
    error::LexerError,
    token::{Token, TokenValue},
};

pub struct Tester {
    tokens: Vec<Token>,
}

impl Tester {
    pub fn into_values_with_positions(self) -> Vec<(TokenValue, Range<usize>)> {
        self.tokens
            .into_iter()
            .map(|token| (token.value, token.address.span))
            .collect()
    }

    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }
}

pub fn code_to_tokens(code: &str) -> Result<Tester, LexerError> {
    let mut lexer = crate::Lexer::new(Source::new("<test>".to_owned(), code.to_owned()).into());

    let mut vec = Vec::with_capacity(8);

    loop {
        match lexer.next_token() {
            Err(LexerError::EOF) => {
                break;
            }
            token_result => {
                vec.push(token_result?);
            }
        }
    }

    Ok(Tester { tokens: vec })
}
