use std::sync::{RwLock};

use dumpster::{Trace, sync::Gc};
use flylang_common::spanned::Spanned;
use flylang_parser::ast::Statement;

use crate::realm::Realm;

#[derive(Clone, Trace)]
pub struct Function {
    pub normal_name: FunctionNameKind,
    pub params: Box<[String]>,
    pub body: Statement,
    pub closure_realm: Gc<RwLock<Realm>>, // captured at definition time
}

#[derive(Clone, Trace)]
pub enum FunctionNameKind {
    Normal(Spanned<String>),
    Anonymous,
}

// It will avoid stack overflowing
impl core::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("params", &self.params)
            .field("body", &self.body)
            .field("closure_env", &"...")
            .finish()
    }
}
