use core::fmt::Debug;

use flylang_common::spanned::Spanned;

pub type Expression = Spanned<ExprKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    // Binary operations
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>, DivisionKind),
    Mod(Box<Expression>, Box<Expression>),

    AddAssign(Box<Expression>, Box<Expression>),
    SubAssign(Box<Expression>, Box<Expression>),
    MulAssign(Box<Expression>, Box<Expression>),
    DivAssign(Box<Expression>, Box<Expression>, DivisionKind),
    ModAssign(Box<Expression>, Box<Expression>),

    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),

    BitAnd(Box<Expression>, Box<Expression>),
    BitOr(Box<Expression>, Box<Expression>),
    BitShiftLeft(Box<Expression>, Box<Expression>),
    BitShiftRight(Box<Expression>, Box<Expression>),

    // Comparison operators
    NotEquals(Box<Expression>, Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    GreaterOrEquals(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    LessOrEquals(Box<Expression>, Box<Expression>),

    // Unary operations
    Not(Box<Expression>),
    Neg(Box<Expression>),

    // Language items
    Identifier(String),
    Number(String),
    String(String),

    Block(Vec<Statement>),
    Array(Vec<Expression>),

    Nil,

    True,
    False,

    Call {
        callee: Box<Expression>,
        parameters: Vec<Expression>,
    },

    Assignment {
        name: Box<Expression>,
        value: Box<Expression>,
    },

    PropertyAccess {
        origin: Box<Expression>,
        property: Box<Expression>,
    },

    Path {
        parent: Box<Expression>,
        value: Box<Expression>,
    },

    IndexedAccess {
        origin: Box<Expression>,
        index: Box<Expression>,
    },

    AnonymousFunction {
        arguments: Vec<Expression>,
        body: Box<Expression>,
    }
}

impl ExprKind {
    pub fn as_id(&self) -> Option<&str> {
        if let ExprKind::Identifier(id) = self {
            Some(id.as_str())
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<&str> {
        if let ExprKind::Number(nr) = self {
            Some(nr.as_str())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Break,
    Continue,

    VariableDefinition(VariableDefinition),

    Function(Function),
    If(If),
    While(While),

    ModuleUsageDeclaration {
        path: Box<Expression>,
    },

    Scope {
        held_value: Box<Expression>,
        body: Box<Statement>,
    },

    Return {
        value: Box<Expression>,
    },

    Expr(Expression),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Spanned<String>,
    pub arguments: Vec<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub body: Box<Statement>,
    pub else_body: Option<Box<Statement>>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Box<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct VariableDefinition {
    pub name: Box<Expression>,
    pub visibility: VariableVisibility,
    pub type_annotation: Option<Box<Expression>>,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum VariableVisibility {
    Local,
    Global
}

#[derive(Debug, Copy, Clone)]
pub enum DivisionKind {
    Neutral,
    RoundingUp,
    RoundingDown,
}
