#[derive(Debug, Clone)]
pub enum Operation {
    AstNode(flylang_parser::ast::Expression)
}
