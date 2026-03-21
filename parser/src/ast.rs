use core::fmt::Display;
use core::fmt::Debug;

use flylang_common::Address;
use flylang_common::spanned::Spanned;

#[derive(Debug, Clone)]
pub enum Expression {
    // Binary operations

    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>, DivisionKind),
    Mod(Box<Expression>, Box<Expression>),

    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),

    BitAnd(Box<Expression>, Box<Expression>),
    BitOr(Box<Expression>, Box<Expression>),
    BitShiftLeft(Box<Expression>, Box<Expression>),
    BitShiftRight(Box<Expression>, Box<Expression>),

    // Comparison operators

    Equals(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    GreaterOrEquals(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    LessOrEquals(Box<Expression>, Box<Expression>),

    // Unary operations

    Not(Box<Expression>),
    Neg(Box<Expression>),

    // Language items

    Identifier(Spanned<String>),
    Number(Spanned<String>),
    String(Spanned<String>),

    Block(Vec<Statement>),
    Array(Vec<Expression>),

    Call {
        callee: Box<Expression>,
        parameters: Vec<Expression>
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        name: Spanned<String>,
        value: Box<Expression>
    },

    MultipleAssignment {
        targets: Vec<Expression>,
        values: Box<Expression>
    },

    VariableDefinition {
        name: Spanned<String>,
        type_annotation: Box<Expression>,
        value: Box<Expression>
    },

    Function {
        name: Spanned<String>,
        arguments: Vec<Expression>,
        body: Box<Statement>
    },

    If {
        condition: Box<Expression>,
        body: Box<Statement>,
        else_body: Option<Box<Statement>>
    },

    Return {
        value: Box<Expression>
    },

    Expr(Expression),
}

#[derive(Debug, Copy, Clone)]
pub enum DivisionKind {
    Neutral,
    RoundingUp,
    RoundingDown
}