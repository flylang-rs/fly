use core::ops::Range;

use core::fmt::Debug;

use dumpster::{Trace, sync::Gc};

pub mod source;
pub mod spanned;
pub mod visibility;

/// An address in source file referencing a token.
#[derive(Clone, Debug, Trace)]
pub struct Address {
    pub source: Gc<source::Source>,
    pub span: Range<usize>,
}

impl Address {
    pub fn merge(self, rhs: &Self) -> Self {
        assert!(
            self.source.filepath == rhs.source.filepath,
            "Tried to add addresses from differenct sources! (`{}` and `{}`)",
            self.source.filepath,
            rhs.source.filepath
        );

        Self {
            source: self.source,
            span: self.span.start..rhs.span.end,
        }
    }
}
