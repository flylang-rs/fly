use num_derive::{FromPrimitive, ToPrimitive};

// NOTE: DON'T change the order of these opcodes, as they are used in the bytecode and must be consistent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum RawOpCode {
    LoadLocal,
    LoadGlobal,
    LoadConst,
    StoreLocal,
    StoreGlobal,
    Add,
    Inc,
    Sub,
    Dec,
    Mul,
    Div,
    CmpWithValue,
    CmpWithConst,
    BranchIfLess,
    BranchIfLessOrEqual,
    BranchIfEqual,
    BranchIfNotEqual,
    BranchIfGreater,
    BranchIfGreaterOrEqual,
    Call,
    Return,
}
