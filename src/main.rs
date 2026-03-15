use std::sync::Arc;

use flylang_lexer::{self, source::Source, token::{Token, TokenValue}};

fn run_file(source: Source) {
    let mut lexer = flylang_lexer::Lexer::new(Arc::new(source));
    let mut tokens: Vec<Token> = vec![];

    while let Some(token) = lexer.next_token() {
        if token.value == TokenValue::Newline {
            println!();
        }
        
        println!("Token: {:?} @ {:?}", token.value, token.address.span);
        tokens.push(token);
    }

    // Parse here...
}

fn main() -> std::io::Result<()> {
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
