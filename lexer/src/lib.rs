pub mod address;
pub mod source;
pub mod token;

use core::{iter::Peekable, ops::Range};
use std::sync::Arc;

use crate::{
    address::Address,
    source::Source,
    token::{Token, TokenValue},
};

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
            .map(|(i, c)| (i, c))
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

    /// Shows an error and bails out.
    fn error(&self, msg: &str, range: Option<&Range<usize>>) -> ! {
        let offset = range.map(|x| x.start).unwrap_or(self.current_offset);
        let len = range.map(|x| x.end - x.start).unwrap_or(1);

        let (line, col) = self.source.location(offset);

        eprintln!(
            "lexer error: {msg}; in {} at line {}; column: {}",
            &self.source.filepath, line, col,
        );

        eprintln!("{:>4} | {}", line, self.source.line_text(line));
        eprintln!("     | {}{}", " ".repeat(col - 1), "^".repeat(len));

        std::process::exit(1);
    }

    /// Lexes an identifier.
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

        (TokenValue::Identifier(id), end)
    }

    /// Lexes a string. Takes `begin_char` as a starting character
    /// Since Fly supports single-quoted (') and double-quoted (") strings, we should differ them.
    pub fn lex_string(&mut self, start: usize, begin_char: char) -> (TokenValue, usize) {
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

                    self.error("EOF while lexing a string", Some(&(start..end)));
                }
            }
        }

        (TokenValue::String(string), end)
    }

    /// Main code: Returns a next token in the code.
    pub fn next_token(&mut self) -> Option<Token> {
        let (position, character) = self.next_character()?;

        // In single-character operations they are all ASCII, so we can safely increment the position.

        let (value, end) = match character {
            '!' => (TokenValue::Bang, position + 1),
            '#' => (TokenValue::Hash, position + 1),
            '=' => (TokenValue::Equals, position + 1),
            '/' => match self.peek_symbol() {
                Some((offset, '=')) => {
                    self.next_character_any();

                    (TokenValue::DivAssign, offset + 1)
                },
                Some((offset, '+')) => {
                    self.next_character_any();

                    match self.peek_symbol() {
                        Some((offset, '=')) => {
                            self.next_character_any();

                            (TokenValue::RoundingUpDivAssign, offset + 1)
                        },
                        _ => (TokenValue::RoundingUpDiv, offset + 1)
                    }
                },
                Some((offset, '-')) => {
                    self.next_character_any();

                    match self.peek_symbol() {
                        Some((offset, '=')) => {
                            self.next_character_any();

                            (TokenValue::RoundingDownDivAssign, offset + 1)
                        },
                        _ => (TokenValue::RoundingDownDiv, offset + 1)
                    }
                }
                _ => (TokenValue::Slash, position + 1),
            },
            '+' => (TokenValue::Plus, position + 1),
            '-' => (TokenValue::Minus, position + 1),
            '\n' => (TokenValue::Newline, position + 1),

            string_start_character @ ('\"' | '\'') => {
                self.lex_string(position, string_start_character)
            }

            _ if character.is_alphabetic() || character == '_' => {
                self.lex_identifier(position, character)
            }

            _ => self.error(&format!("Unknown character: `{}`", character), None),
        };

        Some(self.make_token(value, position..end))
    }
}
