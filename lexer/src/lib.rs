use core::iter::Peekable;

pub struct Lexer {
    source: Peekable<std::vec::IntoIter<char>>,
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
    Newline,

    // There come more complex tokens that consist out of two and more symbols.
    RoundingUpDiv,   // /+
    RoundingDownDiv, // /-

    PlusAssign,            // +=
    MinusAssign,           // -=
    MulAssign,             // *=
    DivAssign,             // /=
    RoundingUpDivAssign,   // /+=
    RoundingDownDivAssign, // /-=
    BitAndAssign,          // &=
    BitOrAssign,           // |=

    LogicalAnd, // &&
    LogicalOr,  // ||
}

#[derive(Debug, Clone)]
pub struct Token {
    value: TokenValue,
    line: usize,
    char: usize,
}

impl Token {
    pub fn new(lexer: &Lexer, value: TokenValue) -> Self {
        Self {
            value,
            line: lexer.position_line,
            char: lexer.position_char,
        }
    }
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        Self {
            source: source_code
                .chars()
                .collect::<Vec<char>>()
                .into_iter()
                .peekable(),
            position_line: 0,
            position_char: 0,
        }
    }

    fn next_symbol_any(&mut self) -> Option<char> {
        let value = self.source.next();

        self.position_char += 1;

        value
    }

    fn peek_symbol(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    fn next_symbol(&mut self) -> Option<char> {
        while let Some(sym) = self.next_symbol_any() {
            if sym == ' ' {
                continue;
            }

            return Some(sym);
        }

        None
    }

    fn error(&self, msg: &str) -> ! {
        eprintln!(
            "lexer error: {msg:?} at line {}; column: {}",
            self.position_line + 1,
            self.position_char + 1
        );

        std::process::exit(1);
    }

    fn lex_string(&mut self) -> TokenValue {
        let mut string = String::new();

        loop {
            let symbol = self.next_symbol_any();

            if symbol == Some('"') {
                break;
            }

            if symbol.is_none() {
                self.error("Unterminated string literal");
            }

            string.push(symbol.unwrap());
        }

        TokenValue::String(string)
    }

    fn lex_identifier(&mut self, beginning_character: char) -> TokenValue {
        let mut id = String::new();

        // We're came from `Lexer::next_token()` method which triggered on an alphanumeric character or `_` symbol.
        // Lexer ate it, so let's add it to final identifier.
        id.push(beginning_character);

        loop {
            let symbol = self.next_symbol_any();

            // If name contains letters, numbers and the `_` symbol, then ...
            if symbol
                .map(|x| x.is_alphanumeric() || x == '_')
                .unwrap_or_default()
            {
                id.push(symbol.unwrap());
            } else {
                break;
            }
        }

        TokenValue::Identifier(id)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let character = self.next_symbol()?;

        let position = (self.position_line, self.position_char);

        let value = match character {
            '!' => TokenValue::Bang,
            '#' => TokenValue::Hash,
            '=' => TokenValue::Equals,
            '\n' => {
                self.position_line += 1;
                self.position_char = 0;

                TokenValue::Newline
            }
            '"' => self.lex_string(),
            _ if (character.is_alphabetic() || character == '_') => self.lex_identifier(character),
            _ => {
                self.error(&format!("Unknown character: `{}`", character));
            }
        };

        Some(Token {
            value,
            line: position.0,
            char: position.1.saturating_sub(1),
        })
    }
}
