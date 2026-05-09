use core::ops::Range;
use core::fmt::Debug;

use dumpster::{Trace, sync::Gc};

/// An address in source file referencing a token.
#[derive(Clone, Trace)]
pub struct Address {
    pub source: Gc<crate::source::Source>,
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

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("<Address `{}`: {:?}>", self.source.filepath, self.span))
    }
}