use flylang_common::Address;

pub struct Diagnostics {
    // ...
}

impl Diagnostics {
    pub fn throw(&self, error: &str, address: &Address) {
        let src = &address.source;
        let location = src.location(address.span.start);

        println!("error: {}:{}:{}: {error} ", src.filepath, location.0, location.1);
        println!("     |");
        println!("{:>4} | {}", location.0 + 1, src.line_text(location.0));
        println!("{:>7}{}", " ".repeat(location.1 + 1), "^".repeat(address.span.end - address.span.start));
    }
}