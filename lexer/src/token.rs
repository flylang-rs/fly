use crate::address::Address;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    // Atoms
    Identifier(String),
    String(String),
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
    Newline,

    Less,
    Greater,

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

    Equals,          // ==
    NotEquals,       // !=
    LessOrEquals,    // <=
    GraeterOrEquals, // >=

    LogicalAnd, // &&
    LogicalOr,  // ||
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: TokenValue,
    pub address: Address,
}
