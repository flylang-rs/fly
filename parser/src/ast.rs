#[derive(Debug, Clone)]
pub enum Node {
    // Binary operations

    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>, DivisionKind),
    Mod(Box<Node>, Box<Node>),

    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),

    BitAnd(Box<Node>, Box<Node>),
    BitOr(Box<Node>, Box<Node>),
    BitShiftLeft(Box<Node>, Box<Node>),
    BitShiftRight(Box<Node>, Box<Node>),

    // Unary operations

    Not(Box<Node>),

    // Language items

    Block(Vec<Node>),

    VariableDefinition {
        name: String,
        type_annotation: Box<Node>,
        value: Box<Node>
    },

    Function {
        name: String,
        arguments: Vec<Node>,
        body: Box<Node>
    },

    Call {
        func_name: String,
        parameters: Vec<Node>
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DivisionKind {
    Neutral,
    RoundingUp,
    RoundingDown
}