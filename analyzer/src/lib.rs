use flylang_parser::ast::{Expression, Statement};

pub struct Analyzer<'a> {
    ast: &'a [Statement]
}

impl<'a> Analyzer<'a> {
    fn analyze(&'a self) {
        for i in self.ast {
            match i {
                Statement::Expr(ex) => self.analyze_expression(ex),
                value => {
                    println!("Value: {value:#?}");
                }
            }
        }

        todo!("Actual analysis");
    }

    fn analyze_function(&self, func: &Statement) {
        
    }

    fn analyze_expression(&self, expression: &Expression) {
        match &expression.value {
            kind => {
                println!("Expression type: {kind:#?}");
            }
        }
    }
}

pub fn analyze(ast: &[Statement]) {
    Analyzer { ast }.analyze()
}