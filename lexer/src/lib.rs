pub mod error;
pub mod kw_lookup_table;
pub mod token;

// Import tests when necessary
#[cfg(test)]
mod tests;

use core::{iter::Peekable, ops::Range};
use std::sync::Arc;

use flylang_common::{Address, source::Source};

use crate::{
    error::LexerError,
    token::{Token, TokenValue},
};

// TODO: Rename it
type LexResult = Result<(TokenValue, usize), error::LexerError>;

pub type LexerResult = Result<Token, error::LexerError>;

/// The lexer.
pub struct Lexer {
    source: Arc<Source>,
    input: Peekable<std::vec::IntoIter<(usize, char)>>,
    current_offset: usize,
}

impl Lexer {
    pub fn new(source: Arc<Source>) -> Self {
        let input = source
            .code
            .char_indices()
            .collect::<Vec<_>>()
            .into_iter()
            .peekable();

        Self {
            source,
            input,
            current_offset: 0,
        }
    }

    /// Just retrieves a character
    fn next_character_any(&mut self) -> Option<(usize, char)> {
        let (offset, ch) = self.input.next()?;

        self.current_offset = offset;

        Some((offset, ch))
    }

    /// Gets current character
    fn peek_symbol(&mut self) -> Option<(usize, char)> {
        self.input.peek().copied()
    }

    /// Retrieves the next non-whitespace character
    fn next_character(&mut self) -> Option<(usize, char)> {
        while let Some(sym @ (_, ch)) = self.next_character_any() {
            // Skip whitespace, but don't skip newlines.
            if ch == ' ' || ch == '\t' {
                continue;
            }

            return Some(sym);
        }

        None
    }

    /// Buils a `Token` out of `TokenValue` and its span.
    fn make_token(&self, value: TokenValue, span: Range<usize>) -> Token {
        Token {
            value,
            address: Address {
                source: Arc::clone(&self.source),
                span,
            },
        }
    }

    /// Lexes an identifier.
    /// Returns a TokenValue::Identifier or one of keywords if matched.
    fn lex_identifier(&mut self, start: usize, first: char) -> (TokenValue, usize) {
        let mut id = String::new();

        // Push the first character to the final id since we ate it in caller fn.
        id.push(first);

        // We're working with raw bytes, so `len_utf8`.
        let mut end = start + first.len_utf8();

        loop {
            match self.peek_symbol() {
                // If current token is alphanumeric or '_', add it to the id.
                Some((offset, ch)) if ch.is_alphanumeric() || ch == '_' => {
                    self.next_character_any();
                    id.push(ch);

                    // Push offset like we did before.
                    end = offset + ch.len_utf8();
                }
                _ => break,
            }
        }

        if let Some(kw) = kw_lookup_table::KEYWORDS.get(id.as_str()) {
            return (kw.clone(), end);
        }

        (TokenValue::Identifier(id), end)
    }

    /// Lexes a string. Takes `begin_char` as a starting character
    /// Since Fly supports single-quoted (') and double-quoted (") strings, we should differ them.
    fn lex_string(&mut self, start: usize, begin_char: char) -> LexResult {
        let mut string = String::new();

        let mut end = start + begin_char.len_utf8();

        loop {
            let current = self.peek_symbol();

            match current {
                Some((offset, character)) => {
                    // If matched, eat it.
                    self.next_character_any();

                    if character == begin_char {
                        break;
                    }

                    string.push(character);

                    end = offset + character.len_utf8();
                }
                None => {
                    // We've (probably) got an EOF.

                    return Err(LexerError::UnexpectedEOF {
                        span: Address {
                            source: self.source.clone(),
                            span: (start..end),
                        },
                    });
                }
            }
        }

        Ok((TokenValue::String(string), end))
    }

    /// Lexes a number
    /// TODO: Floating-point numbers.
    fn lex_number(&mut self, start: usize, first_digit: char) -> LexResult {
        let mut number = String::new();
        number.push(first_digit);

        let mut end = start + first_digit.len_utf8();

        // Determine radix from prefix
        let radix = if first_digit == '0' {
            match self.peek_symbol() {
                Some((_, 'x')) => {
                    self.next_character_any();
                    number.push('x');
                    16
                }
                Some((_, 'o')) => {
                    self.next_character_any();
                    number.push('o');
                    8
                }
                Some((_, 'b')) => {
                    self.next_character_any();
                    number.push('b');
                    2
                }
                _ => 10,
            }
        } else {
            10
        };

        let mut is_a_floating_point_nr = false;

        loop {
            match self.peek_symbol() {
                Some((offset, ch)) if Self::is_digit_for_radix(ch, radix) || ch == '_' => {
                    self.next_character_any();
                    number.push(ch);
                    end = offset + ch.len_utf8();
                }
                Some((offset, '.')) if radix == 10 && !is_a_floating_point_nr => {
                    // If we didn't set the fp number flag, set it, so lexer won't parse
                    // commencing dots anymore.
                    // I mean: the code "3.14159.26" will be lexed into
                    // "3.14159" and "26"
                    // It's actually okay for lexer, but not for parser,
                    // it will handle this gracefully.
                    is_a_floating_point_nr = true;

                    self.next_character_any();
                    number.push('.');
                    // `.` is an ASCII character and its size in Unicode is always 1
                    end = offset + 1;
                }
                Some((offset, ch)) if ch.is_alphabetic() => {
                    return Err(LexerError::InvalidNumberError {
                        span: Address {
                            source: self.source.clone(),
                            span: start..offset + ch.len_utf8(),
                        },
                    });
                }
                Some((offset, ch))
                    if ch.is_ascii_digit() && !Self::is_digit_for_radix(ch, radix) =>
                {
                    return Err(LexerError::InvalidDigitForNumberBase {
                        span: Address {
                            source: self.source.clone(),
                            span: start..offset + ch.len_utf8(),
                        },
                    });
                }
                _ => break, // anything else - delimiter, operator, EOF - stops the number
            }
        }

        Ok((TokenValue::Number(number), end))
    }

    fn is_digit_for_radix(ch: char, radix: u8) -> bool {
        match radix {
            16 => ch.is_ascii_hexdigit(),
            10 => ch.is_ascii_digit(),
            8 => matches!(ch, '0'..='7'),
            2 => matches!(ch, '0' | '1'),
            _ => false,
        }
    }

    /// Lexes `/`, `/=`, `/+`, `/-`, `/+=` and `/-=`
    fn lex_division(&mut self, start: usize) -> (TokenValue, usize) {
        match self.peek_symbol() {
            // /=
            Some((offset, '=')) => {
                self.next_character_any();

                (TokenValue::DivAssign, offset + 1)
            }
            // /+
            Some((offset, '+')) => {
                self.next_character_any();

                match self.peek_symbol() {
                    // /+=
                    Some((offset, '=')) => {
                        self.next_character_any();

                        (TokenValue::RoundingUpDivAssign, offset + 1)
                    }
                    _ => (TokenValue::RoundingUpDiv, offset + 1),
                }
            }
            // /-
            Some((offset, '-')) => {
                self.next_character_any();

                match self.peek_symbol() {
                    // /-=
                    Some((offset, '=')) => {
                        self.next_character_any();

                        (TokenValue::RoundingDownDivAssign, offset + 1)
                    }
                    _ => (TokenValue::RoundingDownDiv, offset + 1),
                }
            }
            // /
            _ => (TokenValue::Slash, start + 1),
        }
    }

    /// Lexes `=` and `==`
    fn lex_equality_sign(&mut self, start: usize) -> (TokenValue, usize) {
        match self.peek_symbol() {
            Some((offset, '=')) => {
                self.next_character_any();

                (TokenValue::Equals, offset + 1)
            }
            _ => (TokenValue::Assign, start + 1),
        }
    }

    /// Lexes `+` and `+=`
    fn lex_plus(&mut self, start: usize) -> (TokenValue, usize) {
        match self.peek_symbol() {
            Some((offset, '=')) => {
                self.next_character_any();

                (TokenValue::PlusAssign, offset + 1)
            }
            _ => (TokenValue::Plus, start + 1),
        }
    }

    /// Lexes `-` and `-=`
    fn lex_minus(&mut self, start: usize) -> (TokenValue, usize) {
        match self.peek_symbol() {
            Some((offset, '=')) => {
                self.next_character_any();

                (TokenValue::MinusAssign, offset + 1)
            }
            _ => (TokenValue::Minus, start + 1),
        }
    }

    /// Lexes `*` and `*=`
    fn lex_asterisk(&mut self, start: usize) -> (TokenValue, usize) {
        match self.peek_symbol() {
            Some((offset, '=')) => {
                self.next_character_any();

                (TokenValue::MulAssign, offset + 1)
            }
            _ => (TokenValue::Asterisk, start + 1),
        }
    }

    /// Lexes only greater or less comparisons (`<`, `>`, `<=`, `>=`)
    fn lex_comparison(&mut self, start: usize, comparator: char) -> (TokenValue, usize) {
        match self.peek_symbol() {
            Some((offset, '=')) => {
                self.next_character_any();

                match comparator {
                    '<' => (TokenValue::LessOrEquals, offset + 1),
                    '>' => (TokenValue::GreaterOrEquals, offset + 1),
                    _ => unreachable!(),
                }
            }
            _ => match comparator {
                '<' => (TokenValue::Less, start + 1),
                '>' => (TokenValue::Greater, start + 1),
                _ => unreachable!(),
            },
        }
    }

    /// Lexes `!` and `!=`
    fn lex_bang(&mut self, start: usize) -> (TokenValue, usize) {
        match self.peek_symbol() {
            Some((offset, '=')) => {
                self.next_character_any();

                (TokenValue::NotEquals, offset + 1)
            }
            _ => (TokenValue::Bang, start + 1),
        }
    }

    fn lex_comment(&mut self, start: usize) -> (TokenValue, usize) {
        let mut comment = String::new();
        let mut end = start + 1;

        loop {
            match self.peek_symbol() {
                Some((offset, char)) => {
                    if char == '\n' {
                        break;
                    }

                    self.next_character_any();

                    comment.push(char);

                    end = offset + char.len_utf8();
                }
                _ => break,
            }
        }

        (TokenValue::Comment(comment), end)
    }

    fn lex_dot(&mut self, position: usize) -> (TokenValue, usize) {
        match self.peek_symbol() {
            Some((offset, '.')) => {
                self.next_character();

                match self.peek_symbol() {
                    Some((offset, '=')) => {
                        self.next_character();

                        (TokenValue::RangeInclusive, offset + 1)
                    }
                    _ => (TokenValue::Range, offset + 1),
                }
            }
            _ => (TokenValue::Dot, position + 1),
        }
    }

    fn lex_ampersand(&mut self, position: usize) -> (TokenValue, usize) {
        match self.peek_symbol() {
            Some((offset, '&')) => {
                self.next_character();

                (TokenValue::LogicalAnd, offset + 1)
            }
            _ => (TokenValue::Ampersand, position + 1),
        }
    }

    fn lex_colon(&mut self, position: usize) -> (TokenValue, usize) {
        match self.peek_symbol() {
            Some((offset, ':')) => {
                self.next_character();

                (TokenValue::PathDelimiter, offset + 1)
            }
            _ => (TokenValue::Colon, position + 1),
        }
    }

    /// Main code: Returns a next token in the code.
    pub fn next_token(&mut self) -> LexerResult {
        let (position, character) = self.next_character().ok_or(LexerError::EOF)?;

        // In single-character operations they are all ASCII, so we can safely increment the position.

        let (value, end) = match character {
            '#' => self.lex_comment(position),
            '!' => self.lex_bang(position),
            '=' => self.lex_equality_sign(position),
            '+' => self.lex_plus(position),
            '-' => self.lex_minus(position),
            '/' => self.lex_division(position),
            '*' => self.lex_asterisk(position),
            '\\' => (TokenValue::Backslash, position + 1),
            '(' => (TokenValue::OpenParen, position + 1),
            ')' => (TokenValue::CloseParen, position + 1),
            '[' => (TokenValue::OpenBracket, position + 1),
            ']' => (TokenValue::CloseBracket, position + 1),
            '{' => (TokenValue::OpenBrace, position + 1),
            '}' => (TokenValue::CloseBrace, position + 1),
            '.' => self.lex_dot(position),
            '&' => self.lex_ampersand(position),
            ',' => (TokenValue::Comma, position + 1),
            ':' => self.lex_colon(position),
            ';' => (TokenValue::Semicolon, position + 1),
            '%' => (TokenValue::Percent, position + 1),
            '\n' => (TokenValue::Newline, position + 1),

            '<' | '>' => self.lex_comparison(position, character),
            '\"' | '\'' => self.lex_string(position, character)?,

            _ if character.is_alphabetic() || character == '_' => {
                self.lex_identifier(position, character)
            }

            _ if character.is_numeric() => self.lex_number(position, character)?,

            _ => Err(LexerError::UnknownCharacter {
                character,
                span: Address {
                    source: self.source.clone(),
                    span: (position..position),
                },
            })?,
        };

        Ok(self.make_token(value, position..end))
    }
}
