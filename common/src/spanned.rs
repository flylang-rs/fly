use crate::Address;

use core::fmt::Debug;

#[derive(Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub address: Address,
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Spanned").field(&self.value).field(&self.address.span).finish()
    }
}