pub struct Lexer {
    source: std::vec::IntoIter<char>,
    position_line: usize,
    position_char: usize,
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    // Atoms

    Identifier(String),
    String(String),
    Equals,
    Plus,
    Minus,
    Asterisk,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Hash,
    Bang,
    QuestionMark,
    Ampersand,
    Percent,
    Slash,
    BackSlash,
    BitAnd,
    BitOr,

    // There come more complex tokens that consist out of two and more symbols.

    RoundingUpDiv,          // /+
    RoundingDownDiv,        // /-

    PlusAssign,             // +=
    MinusAssign,            // -=
    MulAssign,              // *=
    DivAssign,              // /=
    RoundingUpDivAssign,    // /+=
    RoundingDownDivAssign,  // /-=
    BitAndAssign,           // &=
    BitOrAssign,            // |=

    LogicalAnd,          // &&
    LogicalOr,           // ||
}

#[derive(Debug, Clone)]
pub struct Token {
    value: TokenValue,
    line: usize,
    char: usize
}

impl Token {
    pub fn new(lexer: &Lexer, value: TokenValue) -> Self {
        Self {
            value,
            line: lexer.position_line,
            char: lexer.position_char
        }
    }
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        Self {
            source: source_code.chars().collect::<Vec<char>>().into_iter(),
            position_line: 0,
            position_char: 0
        }
    }

    fn next_symbol(&mut self) -> Option<char> {
        self.source.next()
    }

    fn error(&self, msg: &str) -> ! {
        eprintln!("lexer error: {msg:?} at line {}; column: {}", self.position_line + 1, self.position_char + 1);

        std::process::exit(1);
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let character = self.next_symbol()?;

        match character {
            _ => {
                self.error(&format!("Unknown character: {}", character));
            }
        }
    }
}
