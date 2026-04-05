use std::sync::Arc;

use flylang_common::source::Source;
use flylang_diagnostics::additions::Note;
use flylang_lexer::{error::LexerError, token::Token};
use flylang_parser::{Parser, ast::Statement, error::ParserError, state::ParserState};

pub fn parse_source(source: Source) -> Vec<Statement> {
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
                eprintln!("Error: {err:?}");

                std::process::exit(1);
            }
        }
    }

    // Parse here...

    let mut parser = Parser::new(tokens);

    let ast = parser.parse(ParserState::Neutral);

    if let Err(e) = ast {
        // TODO: Move it out
        match e {
            ParserError::UnexpectedEOF => {
                flylang_diagnostics::Diagnostics {}.error(
                    "Unexpected EOF",
                    parser.eof_address(),
                    &[],
                    &[],
                );
            }
            ParserError::UnexpectedTokenInExpression { token } => {
                flylang_diagnostics::Diagnostics {}.error(
                    "Unexpected token",
                    &token.address,
                    &[Note::new(token.address.clone(), "here")],
                    &[],
                );
            }
        }

        // eprintln!("ParserError: {e:#?}");
        std::process::exit(1);
    }

    let ast = ast.unwrap();

    flylang_ast_analyzer::analyze(&ast);

    ast
}