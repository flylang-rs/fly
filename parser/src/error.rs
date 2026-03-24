use flylang_common::Address;
use flylang_lexer::token::Token;

#[derive(Clone, Debug)]
pub enum ParserError {
    UnexpectedEOF,
    UnexpectedTokenInExpression {
        token: Token
    }
}