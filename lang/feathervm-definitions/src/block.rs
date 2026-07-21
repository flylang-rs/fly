use std::fmt::{Debug, Display};

use crate::bytecode::Operation;

pub type Number = String;

#[derive(Debug, Clone)]
pub enum BlockValue {
    Add,
    Mul,
    PushNumber(Number)
}

#[derive(Debug)]
pub enum VMBlock {
    Block {
        code: Vec<BlockValue>
    },
    Single(BlockValue)
}

impl VMBlock {
    pub fn into_single(self) -> Option<BlockValue> {
        match self {
            Self::Single(a) => Some(a),
            _ => None
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
        }.finish()?;

        Ok(())
    }
}