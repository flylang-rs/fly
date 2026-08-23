use std::fmt::{Debug, Display};

use flylang_common::spanned::Spanned;

pub type Number = String;

// TODO: Rename BlockValue to Op, and make a structure that encapsulates it by adding Span to it.
#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Mul,
    Sub,
    Div,
    DivRoundUp,
    DivRoundDown,
    Mod,
    BitShiftLeft,
    BitShiftRight,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    PushNumber(Number),
    PushString(String),
    PushNil,
    Define(String),     // name of definition
    Closure(Closure),   // closure info
    LoadName(String),   // name of object
    Return,
    Call(usize),        // arity
}

pub type BlockValue = Spanned<Op>;

#[derive(Debug, Clone)]
pub enum VMBlock {
    Block { code: Vec<BlockValue> },
    Single(BlockValue),
    Empty,
}

impl VMBlock {
    pub fn into_single(self) -> Option<BlockValue> {
        match self {
            Self::Single(a) => Some(a),
            _ => None,
        }
    }

    pub fn into_content(self) -> Vec<BlockValue> {
        match self {
            Self::Empty => vec![],
            Self::Single(val) => vec![val],
            Self::Block { code } => code,
        }
    }

    pub fn push(&mut self, value: BlockValue) {
        match self {
            VMBlock::Block { code } => code.push(value),
            VMBlock::Single(spanned) => {
                let new = vec![spanned.clone(), value];

                *self = VMBlock::Block { code: new };
            },
            VMBlock::Empty => {
                *self = VMBlock::Single(value);
            },
        }
    }
}

impl Display for VMBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("VMBlock").field("code", &self.code).finish()
        f.write_str("<anonymous>: ")?;

        let mut list = f.debug_list();

        match self {
            VMBlock::Block { code } => list.entries(code),
            VMBlock::Single(block_value) => list.entry(&block_value),
            VMBlock::Empty => list.entry(&Vec::<BlockValue>::new()),
        }
        .finish()?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub body: Vec<VMBlock>,
    pub arguments: Vec<String>,
}