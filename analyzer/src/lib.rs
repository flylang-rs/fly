use flylang_parser::ast::Statement;

pub struct Analyzer<'a> {
    ast: &'a Statement
}