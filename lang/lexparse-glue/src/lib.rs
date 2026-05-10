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
    AnalyzeFailed,
}

impl DiagnosticsReport for LoadingError {
    fn render(&self) -> String {
        match self {
            LoadingError::LexerError(lexer_error) => lexer_error.render(),
            LoadingError::ParserError(parser_error) => parser_error.render(),
            LoadingError::AnalyzeFailed => unreachable!("Not rendered as diagnostics"),
        }
    }
}

pub fn lex_source(source: Arc<Source>, make_error: bool) -> LoadingResult<Vec<Token>> {
    let mut lexer = flylang_lexer::Lexer::new(Arc::clone(&source));
    let mut tokens: Vec<Token> = vec![];

    loop {
        match lexer.next_token() {
            // If we got a token, add it
            Ok(token) => tokens.push(token),
            // It's actually not an error, it indicated that we reached the end, so just
            Err(LexerError::EOF) => break,
            // This arm is more serious, we have a problem here
            Err(err) => {
                if make_error {
                    return Err(LoadingError::LexerError(err));
                } else {
                    // If make_error == false, return what we have.
                    // This behaviour is useful for REPL, since input coloring is based on lexer...
                    // ... any error will remove all coloring, so we're doing some "error silencer" here.
                    return Ok(tokens);
                }
            }
        }
    }

    Ok(tokens)
}

pub fn parse_source(source: Arc<Source>) -> LoadingResult<Vec<Statement>> {
    let tokens = lex_source(Arc::clone(&source), true)?;

    // Parse here...

    let mut parser = Parser::new(tokens, &source);

    let ast = parser.parse(ParserState::Neutral);

    if let Err(e) = ast {
        return Err(LoadingError::ParserError(e));
    }

    let ast = ast.unwrap();

    let analyzer = flylang_ast_analyzer::analyze(&ast);

    if analyzer.error_count() != 0 {
        return Err(LoadingError::AnalyzeFailed);
    }

    Ok(ast)
}
