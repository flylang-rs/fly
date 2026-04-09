use flylang_common::spanned::Spanned;

use flylang_common::Address;

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
    Percent,
    Slash,
    BackSlash,
    BitAnd,
    BitOr,
    BitShiftLeft,
    BitShiftRight,
    Backslash,
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
    Nil,
    Operator,
    Override,
    Private,
    Public,
    Record,
    Return,
    SelfRecord,
    SelfReference,
    Static,
    True,
    Use,
    While,
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
