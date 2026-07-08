use std::fmt::{Debug, Display};

use crate::bytecode::RawOpCode;

/// Represents an opcode or an operand for it.
#[derive(Debug)]
pub enum BlockValue {
    Opcode(RawOpCode),
    ByteValue(u8),
    ShortValue(u16),
    WordValue(u32),
    ShortIntegerConstant(i32),
}

#[derive(Debug)]
pub struct VMBlock {
    code: Vec<BlockValue>
}

impl Display for VMBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("VMBlock").field("code", &self.code).finish()
        f.write_str("<anonymous>: ")?;
        
        f.debug_list().entries(&self.code).finish()?;

        Ok(())
    }
}