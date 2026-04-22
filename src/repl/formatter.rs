use flylang_common::{Address, source::Source};
use flylang_lexer::token::{Token, TokenValue};
use flylang_lexparse_glue::LoadingResult;

use owo_colors::{OwoColorize, Stream};

pub struct REPLFormatter;

pub struct REPLToken<'a> {
    kind: TokenValue,
    address: Address,
    raw_value: &'a str,
}

impl REPLFormatter {
    pub fn format(code: &str) -> LoadingResult<String> {
        let tokens = flylang_lexparse_glue::lex_source(
            Source::new("<REPL>".to_string(), code.to_string()).into(),
        )?;

        let result = tokens
            .into_iter()
            .map(|Token { value, address }| REPLToken {
                kind: value,
                raw_value: &code[address.span.clone()],
                address: address,
            });

        let mut finale = code.to_string();

        let mut shift = 0_usize;

        for i in result {
            let replacement = match i.kind {
                TokenValue::String(_) => owo_colors::AnsiColors::Yellow,
                TokenValue::Comment(_) => owo_colors::AnsiColors::BrightBlack,
                TokenValue::Number(_) => owo_colors::AnsiColors::Cyan,
                TokenValue::OpenParen | TokenValue::CloseParen => owo_colors::AnsiColors::Blue,
                TokenValue::OpenBrace | TokenValue::CloseBrace => owo_colors::AnsiColors::Blue,
                TokenValue::OpenBracket | TokenValue::CloseBracket => owo_colors::AnsiColors::Blue,
                tok if TokenValue::is_keyword(&tok) => owo_colors::AnsiColors::Magenta,
                _ => continue,
            };

            let replacement = i
                .raw_value
                .if_supports_color(Stream::Stdout, |x| x.color(replacement))
                .to_string();

            let orig_len = i.raw_value.len();
            let replacement_len = replacement.len();

            let final_range = i.address.span.start + shift..i.address.span.end + shift;

            finale.replace_range(final_range, &replacement);

            shift += replacement_len - orig_len;
        }

        Ok(finale)
    }
}
