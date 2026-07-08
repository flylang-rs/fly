use std::{ops::{Add, Deref}, sync::Arc};

use dumpster::{TraceWith, Visitor};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FlyString {
    _ref: Arc<String>
}

impl FlyString {
    pub fn new(s: String) -> Self {
        Self {
            _ref: s.into()
        }
    }

    pub fn refcount(&self) -> usize {
        Arc::strong_count(&self._ref)
    }
}

impl Add for FlyString {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            _ref: ((*self._ref).clone() + &rhs._ref).into()
        }
    }
}

impl Deref for FlyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self._ref
    }
}

impl From<String> for FlyString {
    fn from(value: String) -> Self {
        Self { _ref: value.into() }
    }
}

impl From<&str> for FlyString {
    fn from(value: &str) -> Self {
        Self { _ref: Arc::new(value.into()) }
    }
}

// SAFETY: Reference cycles cannot happen within `::alloc::string::String` type.
unsafe impl<V: Visitor> TraceWith<V> for FlyString {
    fn accept(&self, _visitor: &mut V) -> Result<(), ()> {
        Ok(())
    }
}