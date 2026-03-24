use flylang_diagnostics::Diagnostics;
use flylang_parser::ast::{ExprKind, Expression, Function, Statement};

pub struct Analyzer<'a> {
    ast: &'a [Statement],
    error_count: usize,
    warning_count: usize,
}

impl<'a> Analyzer<'a> {
    pub fn new(ast: &'a [Statement]) -> Self {
        Self {
            ast,
            error_count: 0,
            warning_count: 0
        }
    }

    pub fn error_count(&self) -> usize {
        self.error_count
    }

    pub fn warning_count(&self) -> usize {
        self.error_count
    }

    pub fn analyze(&'a mut self) {
        for i in self.ast {
            match i {
                Statement::Expr(ex) => self.analyze_expression(ex),
                Statement::Function(func) => self.analyze_function(func),

                // A lot things to analyze.
                _ => ()
            }
        }
    }

    fn analyze_function(&mut self, _func: &Function) {
        // IDK what to analyze here yet.
        // ...
    }

    fn analyze_expression(&mut self, expression: &Expression) {
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
                    Diagnostics{}.throw("This kind of expression is not allowed as LHS.", &name.address);

                    self.error_count += 1;
                }
            }
            kind => {
                println!("Expression type: {kind:#?}");
            }
        }
    }
}

pub fn analyze(ast: &[Statement]) {
    let mut analyzer = Analyzer::new(ast);
    
    analyzer.analyze()
}
