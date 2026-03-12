use core::ops::Range;
use std::sync::Arc;

use crate::source::Source;

/// An address in source file referencing a token.
#[derive(Clone, Debug)]
pub struct Address {
    pub source: Arc<Source>,
    pub span: Range<usize>,
}