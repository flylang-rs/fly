use std::sync::Arc;

use flylang_common::source::Source;
use flylang_diagnostics::error::DiagnosticsReport;
use flylang_lexer::{error::LexerError, token::Token};
use flylang_parser::{Parser, ast::Statement, error::ParserError, state::ParserState};

#[derive(Debug, Clone)]
pub enum ImportError {
    LexerError(LexerError),
    ParserError(ParserError),
    AnalyzeFailed,
}

impl DiagnosticsReport for ImportError {
    fn render(&self) -> String {
        match self {
            ImportError::LexerError(lexer_error) => lexer_error.render(),
            ImportError::ParserError(parser_error) => parser_error.render(),
            ImportError::AnalyzeFailed => unreachable!("Not rendered as diagnostics"),
        }
    }
}

pub fn parse_into_ast(source: Source) -> Result<Vec<Statement>, ImportError> {
    let mut lexer = flylang_lexer::Lexer::new(Arc::new(source));
    let mut tokens: Vec<Token> = vec![];

    loop {
        match lexer.next_token() {
            // If we got a token, add it
            Ok(token) => tokens.push(token),
            // It's actually not an error, it indicated that we reached the end, so just break
            Err(LexerError::EOF) => break,
            // This arm is more serious, we have a problem here
            Err(err) => {
                return Err(ImportError::LexerError(err));
            }
        }
    }

    // Parse here...

    let mut parser = Parser::new(tokens);

    let ast = parser.parse(ParserState::Neutral);

    if let Err(e) = ast {
        return Err(ImportError::ParserError(e));
    }

    let ast = ast.unwrap();

    let analyzer = flylang_ast_analyzer::analyze(&ast);

    if analyzer.error_count() != 0 {
        return Err(ImportError::AnalyzeFailed);
    }

    Ok(ast)
}
