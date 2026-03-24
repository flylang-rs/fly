use flylang_parser::ast::{ExprKind, Expression, Function, Statement};

pub struct Analyzer<'a> {
    ast: &'a [Statement],
}

impl<'a> Analyzer<'a> {
    fn analyze(&'a self) {
        for i in self.ast {
            match i {
                Statement::Expr(ex) => self.analyze_expression(ex),
                Statement::Function(func) => self.analyze_function(func),

                // A lot things to analyze.
                _ => ()
            }
        }
    }

    fn analyze_function(&self, _func: &Function) {
        // IDK what to analyze here yet.
        // ...
    }

    fn analyze_expression(&self, expression: &Expression) {
        match &expression.value {
            ExprKind::Assignment { name, .. } => {
                // LHS should be an identifier (abc), indexed access (abc[2]), property access (fly.with.me) for single-target operations
                // Like this:
                //   abc = [1, 2, 3, 4]
                //   abc[2] = 0
                //   fly.with.me = true
                //
                // LHS should be an array definition for multi-target operation
                // Like this:
                //   [a, b] = [b, a]   # Swap variables `a` and `b`
                if !matches!(
                    name.value,
                    ExprKind::Identifier(_)
                        | ExprKind::Array(_)
                        | ExprKind::IndexedAccess { .. }
                        | ExprKind::PropertyAccess { .. }
                ) {
                    panic!(
                        "This kind of expression is not allowed as LHS. (at {:?})",
                        name.address
                    );
                }
            }
            kind => {
                println!("Expression type: {kind:#?}");
            }
        }
    }
}

pub fn analyze(ast: &[Statement]) {
    Analyzer { ast }.analyze()
}
