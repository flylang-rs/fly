use std::sync::{Arc, RwLock};

use flylang_parser::ast::Statement;

use crate::realm::Realm;

#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Statement,
    pub closure_realm: Arc<RwLock<Realm>>, // captured at definition time
}

impl core::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("params", &self.params)
            .field("body", &self.body)
            .field("closure_env", &"...")
            .finish()
    }
}
