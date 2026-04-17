use crate::{ParserResult, ast::Statement};

pub fn code2ast(code: &str) -> ParserResult<Vec<Statement>> {
    let tokens = flylang_lexer::test_utils::code_to_tokens(code)
        .unwrap()
        .into_tokens();

    let mut parser = crate::Parser::new(tokens);

    parser.parse(crate::state::ParserState::Neutral)
}
