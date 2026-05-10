use core::ops::Range;
use core::fmt::Debug;
use std::sync::Arc;

use dumpster::{TraceWith, Visitor};

/// An address in source file referencing a token.
#[derive(Clone)]
pub struct Address {
    pub source: Arc<crate::source::Source>,
    pub span: Range<usize>,
}

// SAFETY: `Address::source` can't make reference cycles, so `Arc` is acceptable here.
unsafe impl<V: Visitor> TraceWith<V> for Address {
    fn accept(&self, _visitor: &mut V) -> Result<(), ()> {
        Ok(())
    }
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