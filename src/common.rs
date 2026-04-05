use std::sync::Arc;

use flylang_common::source::Source;
use flylang_diagnostics::error::DiagnosticsReport;
use flylang_lexer::{error::LexerError, token::Token};
use flylang_parser::{Parser, ast::Statement, error::ParserError, state::ParserState};

pub type LoadingResult<T> = Result<T, LoadingError>;

#[derive(Debug, Clone)]
pub enum LoadingError {
    LexerError(LexerError),
    ParserError(ParserError),
}

impl DiagnosticsReport for LoadingError {
    fn render(&self) -> String {
        match self {
            LoadingError::LexerError(lexer_error) => lexer_error.render(),
            LoadingError::ParserError(parser_error) => parser_error.render(),
        }
    }
}

pub fn parse_source(source: Source) -> LoadingResult<Vec<Statement>> {
    let source = Arc::new(source);
    
    let mut lexer = flylang_lexer::Lexer::new(source);
    let mut tokens: Vec<Token> = vec![];

    loop {
        match lexer.next_token() {
            // If we got a token, add it
            Ok(token) => tokens.push(token),
            // It's actually not an error, it indicated that we reached the end, so just break
            Err(LexerError::EOF) => break,
            // This arm is more serious, we have a problem here
            Err(err) => {
                return Err(LoadingError::LexerError(err));
            }
        }
    }

    // Parse here...

    let mut parser = Parser::new(tokens);

    let ast = parser.parse(ParserState::Neutral);

    if let Err(e) = ast {
        return Err(LoadingError::ParserError(e));
    }

    let ast = ast.unwrap();

    flylang_ast_analyzer::analyze(&ast);

    Ok(ast)
}