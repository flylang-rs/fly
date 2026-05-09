use core::fmt::Debug;
use dumpster::Trace;

use crate::address::Address;

#[derive(Clone, Trace)]
pub struct Spanned<T: Trace> {
    pub value: T,
    pub address: Address,
}

impl<T: Trace> Spanned<T> {
    pub fn new(value: T, address: Address) -> Self {
        Self { value, address }
    }

    pub fn map<R: Trace>(self, f: impl FnOnce(T) -> R) -> Spanned<R> {
        Spanned {
            value: f(self.value),
            address: self.address.clone(),
        }
    }
}

impl<T: Debug + Trace> Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Spanned")
            .field(&self.value)
            .field(&self.address.span)
            .finish()
    }
}
