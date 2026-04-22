use flylang_common::spanned::Spanned;

use flylang_common::Address;

use crate::kw_lookup_table;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    // Atoms
    Identifier(String),
    String(String),
    Comment(String),
    Number(String),
    Assign,
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
    Bar,
    Percent,
    Slash,
    Backslash,
    BitShiftLeft,
    BitShiftRight,
    Dot,
    Comma,
    Colon,
    Semicolon,

    Newline,

    Less,
    Greater,

    // There come more complex tokens that consist out of two and more symbols.
    ArrowForward,  // ->
    PathDelimiter, // ::

    Range,          // ..
    RangeInclusive, // ..=

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
    PercentAssign,         // %=

    Equals,          // ==
    NotEquals,       // !=
    LessOrEquals,    // <=
    GreaterOrEquals, // >=

    LogicalAnd, // &&
    LogicalOr,  // ||

    // Keywords
    Break,
    Continue,
    Destructor,
    Drop,
    Else,
    False,
    For,
    Func,
    If,
    New,
    Nil,
    Operator,
    Override,
    Private,
    Public,
    Record,
    Return,
    SelfRecord,
    Static,
    True,
    Use,
    While,
}

impl TokenValue {
    pub fn repr(&self) -> &str {
        match self {
            TokenValue::Identifier(id) => id,
            TokenValue::String(st) => st,
            TokenValue::Comment(com) => com,
            TokenValue::Number(nr) => nr,
            TokenValue::Assign => "=",
            TokenValue::Plus => "+",
            TokenValue::Minus => "-",
            TokenValue::Asterisk => "*",
            TokenValue::OpenBrace => "{",
            TokenValue::CloseBrace => "}",
            TokenValue::OpenBracket => "[",
            TokenValue::CloseBracket => "]",
            TokenValue::OpenParen => "(",
            TokenValue::CloseParen => ")",
            TokenValue::Hash => "#",
            TokenValue::Bang => "!",
            TokenValue::QuestionMark => "?",
            TokenValue::Ampersand => "&",
            TokenValue::Bar => "|",
            TokenValue::Percent => "%",
            TokenValue::Slash => "/",
            TokenValue::Backslash => "\\",
            TokenValue::BitShiftLeft => "<<",
            TokenValue::BitShiftRight => ">>",
            TokenValue::Dot => ".",
            TokenValue::Comma => ",",
            TokenValue::Colon => ":",
            TokenValue::Semicolon => ";",
            TokenValue::Newline => "<newline>",
            TokenValue::Less => "<",
            TokenValue::Greater => ">",
            TokenValue::ArrowForward => "->",
            TokenValue::PathDelimiter => "::",
            TokenValue::Range => "..",
            TokenValue::RangeInclusive => "..=",
            TokenValue::RoundingUpDiv => "/+",
            TokenValue::RoundingDownDiv => "/-",
            TokenValue::PlusAssign => "+=",
            TokenValue::MinusAssign => "-",
            TokenValue::MulAssign => "*",
            TokenValue::DivAssign => "/",
            TokenValue::RoundingUpDivAssign => "/+=",
            TokenValue::RoundingDownDivAssign => "/-=",
            TokenValue::BitAndAssign => "&=",
            TokenValue::BitOrAssign => "|=",
            TokenValue::PercentAssign => "%=",
            TokenValue::Equals => "==",
            TokenValue::NotEquals => "!=",
            TokenValue::LessOrEquals => "<=",
            TokenValue::GreaterOrEquals => ">=",
            TokenValue::LogicalAnd => "&&",
            TokenValue::LogicalOr => "||",

            token => {
                // If it's a keyword, find it in keyword lookup table instead.

                let value = kw_lookup_table::tokenvalue_to_name(token);

                if let Some(value) = value {
                    return value;
                }

                unimplemented!(
                    "Cannot transform token value: {token:?} to its string representation."
                )
            }
        }
    }

    pub fn is_keyword(&self) -> bool {
        kw_lookup_table::tokenvalue_to_name(self).is_some()
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: TokenValue,
    pub address: Address,
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        matches!(self.value, TokenValue::Identifier(_))
    }

    pub fn into_spanned_identifier(self) -> Option<Spanned<String>> {
        match self {
            Token {
                value: TokenValue::Identifier(id),
                address,
            } => Some(Spanned { value: id, address }),
            _ => None,
        }
    }
}
