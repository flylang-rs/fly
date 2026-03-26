use flylang_common::Address;
use flylang_parser::ast::{ExprKind, Statement};

pub struct Note<'a> {
    pub(crate) position: Address,
    pub(crate) message: &'a str,
}

impl<'a> Note<'a> {
    pub fn new(position: Address, message: &'a str) -> Self {
        Self { position, message }
    }
}

pub struct Help<'a> {
    pub(crate) message: &'a str,
    pub(crate) new_ast: Statement,
}

impl<'a> Help<'a> {
    pub fn new(message: &'a str, new_ast: Statement) -> Self {
        Self { message, new_ast }
    }

    fn build_code_inner(&self, ast: Statement) -> String {
        match ast {
            Statement::Expr(spanned) => match spanned.value {
                ExprKind::Assignment { name, value } => {
                    return self.build_code_inner(Statement::Expr(*name))
                        + " = "
                        + &self.build_code_inner(Statement::Expr(*value));
                }
                ExprKind::Identifier(id) => return id,
                ExprKind::Number(nr) => return nr,
                ExprKind::String(string) => return string,
                _ => todo!("Implement code building for other expression kinds for diagnostics."),
            },
            _ => todo!("Implement code building for other types for diagnostics."),
        }
    }

    pub fn build_code(&self) -> String {
        self.build_code_inner(self.new_ast.clone())
    }
}
