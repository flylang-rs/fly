use core::ops::Range;

use flylang_common::Address;

pub struct Note<'a> {
    pub(crate) position: Option<Address>,
    pub(crate) message: &'a str,
}

impl<'a> Note<'a> {
    pub fn new(position: Address, message: &'a str) -> Self {
        Self { position: Some(position), message }
    }

    pub fn message(message: &'a str) -> Self {
        Self { position: None, message }
    }

}

#[derive(Debug, Clone)]
pub struct TextEdit {
    pub span: Range<usize>,
    pub replacement: Option<String>,
}

impl TextEdit {
    pub fn new(span: Range<usize>, replacement: String) -> Self {
        Self {
            span,
            replacement: Some(replacement),
        }
    }

    pub fn delete(span: Range<usize>) -> Self {
        Self {
            span,
            replacement: None,
        }
    }
}

pub struct Help<'a> {
    pub(crate) message: &'a str,
    pub(crate) edits: Vec<TextEdit>,
}

impl<'a> Help<'a> {
    pub fn new(message: &'a str, edits: Vec<TextEdit>) -> Self {
        Self { message, edits }
    }
}
