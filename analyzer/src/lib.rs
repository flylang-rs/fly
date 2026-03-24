use flylang_parser::ast::Statement;

pub struct Analyzer<'a> {
    ast: &'a [Statement]
}

impl<'a> Analyzer<'a> {
    fn analyze(&'a self) {
        todo!("Actual analysis");
    }
}

pub fn analyze(ast: &[Statement]) {
    Analyzer { ast }.analyze()
}