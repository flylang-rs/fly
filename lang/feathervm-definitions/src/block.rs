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
    Define(String),
    Closure(Closure),
    LoadName(String),
}

pub type BlockValue = Spanned<Op>;

#[derive(Debug)]
pub enum VMBlock {
    Block { code: Vec<BlockValue> },
    Single(BlockValue),
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
            Self::Single(val) => vec![val],
            Self::Block { code } => code
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
        }
        .finish()?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Closure {

}