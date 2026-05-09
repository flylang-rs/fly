use dumpster::sync::Gc;
use flylang_common::source::Source;

use crate::{ParserResult, ast::Statement};

pub fn code2ast(code: &str) -> ParserResult<Vec<Statement>> {
    let tokens = flylang_lexer::test_utils::code_to_tokens(code)
        .unwrap()
        .into_tokens();

    let source: Gc<Source> = Source::new("<test>".to_owned(), code.to_owned()).into();

    let mut parser = crate::Parser::new(tokens, &source);

    parser.parse(crate::state::ParserState::Neutral)
}
