use std::sync::Arc;

use flylang_common::source::Source;
use flylang_diagnostics::additions::Note;
use flylang_lexer::{
    self,
    error::LexerError,
    token::{Token, TokenValue},
};
use flylang_parser::{Parser, error::ParserError, state::ParserState};

fn run_file(source: Source) {
    let mut lexer = flylang_lexer::Lexer::new(Arc::new(source));
    let mut tokens: Vec<Token> = vec![];

    loop {
        match lexer.next_token() {
            // If we got a token, add it
            Ok(token) => {
                if token.value == TokenValue::Newline {
                    println!();
                }

                println!("Token: {:?} @ {:?}", token.value, token.address.span);
                tokens.push(token);
            }
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

    println!();
    println!("----- AST -----");
    println!();

    if let Err(e) = ast {
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

    for (n, i) in ast.iter().enumerate() {
        println!("[{n}]: {:#?}", i);
    }

    flylang_ast_analyzer::analyze(&ast);

    let interpreter = flylang_tte::Interpreter::new();

    let result = interpreter.execute_script(ast);

    println!("Program finished with result: {result:?}");
}

fn main() -> std::io::Result<()> {
    env_logger::builder()
        .default_format()
        .format_timestamp_millis()
        .init();

    let filepath = if let Some(arg) = std::env::args().nth(1) {
        arg
    } else {
        eprintln!("No file specified!");

        std::process::exit(1);
    };

    let source_code = std::fs::read_to_string(&filepath)?;

    run_file(Source::new(filepath, source_code));

    Ok(())
}
