use std::sync::Arc;

use fly_lexer::{self, source::Source};

fn main() -> std::io::Result<()> {
    let filepath = if let Some(arg) = std::env::args().nth(1) {
        arg
    } else {
        eprintln!("No file specified!");

        std::process::exit(1);
    };

    let source_code = std::fs::read_to_string(&filepath)?;

    let mut lexer = fly_lexer::Lexer::new(Arc::new(Source::new(filepath, source_code)));

    while let Some(token) = lexer.next_token() {
        println!("Token: {:?} @ {:?}", token.value, token.address.span);
    }

    Ok(())
}
