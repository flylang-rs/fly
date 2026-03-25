use flylang_common::Address;

use owo_colors::OwoColorize;

pub struct Diagnostics {
    // ...
}

impl Diagnostics {
    pub fn error(&self, error: &str, address: &Address) {
        let src = &address.source;
        let location = src.location(address.span.start);

        println!("{}: {}:{}:{}: {} ", "error".bold().red(), src.filepath, location.0, location.1, error.bold());
        println!("     |");
        println!("{:>4} | {}", location.0 + 1, src.line_text(location.0));
        println!("{:>4} | {}", " ".repeat(location.1 + 1), "^".repeat(address.span.end - address.span.start));
        println!("     |");
    }
}
