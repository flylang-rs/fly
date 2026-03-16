use core::ops::Range;
use std::sync::Arc;

use core::fmt::Debug;

pub mod source;
pub mod spanned;

/// An address in source file referencing a token.
#[derive(Clone, Debug)]
pub struct Address {
    pub source: Arc<source::Source>,
    pub span: Range<usize>,
}