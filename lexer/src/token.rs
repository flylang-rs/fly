use crate::address::Address;

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
    pub value: TokenValue,
    pub address: Address,
}