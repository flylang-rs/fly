use core::ops::Range;

use crate::{source::Source, token::{Token, TokenValue}};

pub(super) struct Tester {
    tokens: Vec<Token>
}

impl Tester {
    pub fn into_values_with_positions(self) -> Vec<(TokenValue, Range<usize>)> {
        self.tokens.into_iter().map(|token| (token.value, token.address.span)).collect()
    }
}

pub(super) fn code_to_tokens(code: &str) -> Tester {
    let mut lexer = crate::Lexer::new(
        Source::new(
            "test.fly".to_owned(),
            code.to_owned()
        ).into()
    );

    let mut vec = Vec::with_capacity(8);

    while let Some(token) = lexer.next_token() {
        vec.push(token);
    }

    Tester { tokens: vec }
}