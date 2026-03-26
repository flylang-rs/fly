use flylang_common::spanned::Spanned;
use flylang_diagnostics::{
    Diagnostics,
    additions::{Help, Note},
};
use flylang_parser::ast::{ExprKind, Expression, Function, Statement};

pub struct Analyzer<'a> {
    ast: &'a [Statement],
    diag: Diagnostics,
    error_count: usize,
    warning_count: usize,
}

impl<'a> Analyzer<'a> {
    pub fn new(ast: &'a [Statement]) -> Self {
        Self {
            ast,
            diag: Diagnostics {},
            error_count: 0,
            warning_count: 0,
        }
    }

    pub fn error_count(&self) -> usize {
        self.error_count
    }

    pub fn warning_count(&self) -> usize {
        self.warning_count
    }

    pub fn analyze(&'a mut self) {
        for i in self.ast {
            match i {
                Statement::Expr(ex) => self.analyze_expression(ex),
                Statement::Function(func) => self.analyze_function(func),

                // A lot things to analyze.
                _ => (),
            }
        }
    }

    fn analyze_function(&mut self, _func: &Function) {
        // IDK what to analyze here yet.
        // ...
    }

    fn analyze_expression(&mut self, expression: &Expression) {
        match &expression.value {
            ExprKind::Assignment { name, value } => {
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
                    self.diag.error(
                        "This kind of expression is not allowed as LHS.",
                        &name.address,
                        &[
                            Note::new(name.address.clone(), "there")
                        ],
                        &[]
                    );

                    self.error_count += 1;
                }

                // If we have an assignment expression like this:
                // [a, b, ..., z, ...] = [value1, value2, ..., value_n, ...]
                if let ExprKind::Array(lhs_arr) = &name.value
                    && let ExprKind::Array(rhs_arr) = &value.value
                {
                    // And their length are 1, it will mean that dev tried to make multiple assignment
                    // with only one target which is redunant and can be reduced:
                    // [a] = [4]    # ->  a = 4
                    if lhs_arr.len() == 1 && rhs_arr.len() == 1 {
                        // self.diag.warning("Redundant multiple assignment", &name.address);
                        // self.diag.warning("Redundant multiple assignment", &value.address);

                        self.diag.warning(
                            "Redundant multiple assignment",
                            &name.address,
                            &[
                                Note::new(name.address.clone(), "one element here"),
                                Note::new(value.address.clone(), "one element there"),
                            ],
                            &[Help::new(
                                "reduce it",
                                Statement::Expr(Spanned {
                                    value: ExprKind::Assignment {
                                        name: Box::new(lhs_arr[0].clone()),
                                        value: Box::new(rhs_arr[0].clone()),
                                    },
                                    address: expression.address.clone(),
                                }),
                            )],
                        );
                    }

                    println!("Both arrays!");
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
