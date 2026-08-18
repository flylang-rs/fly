use feathervm_definitions::{
    block::{BlockValue, Op, VMBlock},
};
use flylang_common::spanned::Spanned;
use flylang_parser::{
    ast::{
        DivisionKind, ExprKind, Expression, Function,
        Statement::{self, Expr},
    },
    state,
};

mod value;

pub struct Compiler {
    global_compile_time_values: value::CompileTimeValues,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            global_compile_time_values: value::CompileTimeValues::new(),
        }
    }

    pub fn compile(&self, ast: &[flylang_parser::ast::Statement]) -> Result<Vec<VMBlock>, String> {
        // Placeholder for the actual compilation logic.
        // In a real implementation, this would involve parsing the source code,
        // generating bytecode, and returning it as a vector of bytes.
        let mut blocks = vec![];

        for i in ast {
            let block = self.compile_statement(i)?;

            blocks.push(block);
        }

        Ok(blocks)
    }

    fn compile_statement(&self, statement: &Statement) -> Result<VMBlock, String> {
        match statement {
            Expr(spanned) => self.compile_expr(spanned),
            Statement::Break => todo!(),
            Statement::Continue => todo!(),
            Statement::VariableDefinition(variable_definition) => todo!(),
            Statement::Function(function) => self.compile_function(function),
            Statement::If(_) => todo!(),
            Statement::While(_) => todo!(),
            Statement::RecordDefinition(record_definition) => todo!(),
            Statement::ModuleUsageDeclaration { path } => todo!(),
            Statement::Scope { held_value, body } => todo!(),
            Statement::Return { value } => todo!(),
        }
    }

    fn load_value(&self, expr: &Expression) -> Result<Vec<BlockValue>, String> {
        let value = self.compile_expr(expr)?;

        match value {
            VMBlock::Block { code } => Ok(code),
            VMBlock::Single(block_value) => Ok(vec![block_value]),
        }
    }

    fn compile_expr(&self, statement: &Expression) -> Result<VMBlock, String> {
        match &statement.value {
            ExprKind::Add(a, b) => {
                let value_a = self.load_value(a)?;
                let value_b = self.load_value(b)?;

                let mut result = vec![];

                result.extend_from_slice(&value_a);
                result.extend_from_slice(&value_b);

                result.push(Spanned::new(Op::Add, statement.address.clone()));

                Ok(VMBlock::Block { code: result })
            }
            ExprKind::Mul(a, b) => {
                let value_a = self.load_value(a)?;
                let value_b = self.load_value(b)?;

                let mut result = vec![];

                result.extend_from_slice(&value_a);
                result.extend_from_slice(&value_b);

                result.push(Spanned::new(Op::Mul, statement.address.clone()));

                Ok(VMBlock::Block { code: result })
            }
            ExprKind::Div(a, b, dk) => {
                let value_a = self.load_value(a)?;
                let value_b = self.load_value(b)?;

                let mut result = vec![];

                result.extend_from_slice(&value_a);
                result.extend_from_slice(&value_b);

                let bv = match dk {
                    DivisionKind::Neutral => Op::Div,
                    DivisionKind::RoundingUp => Op::DivRoundUp,
                    DivisionKind::RoundingDown => Op::DivRoundDown,
                };

                result.push(Spanned::new(bv, statement.address.clone()));

                Ok(VMBlock::Block { code: result })
            }
            ExprKind::Sub(a, b) => {
                let value_a = self.load_value(a)?;
                let value_b = self.load_value(b)?;

                let mut result = vec![];

                result.extend_from_slice(&value_a);
                result.extend_from_slice(&value_b);

                result.push(Spanned::new(Op::Sub, statement.address.clone()));

                Ok(VMBlock::Block { code: result })
            }
            ExprKind::Number(nr) => Ok(VMBlock::Single(Spanned::new(
                Op::PushNumber(nr.clone()),
                statement.address.clone(),
            ))),
            ExprKind::String(st) => Ok(VMBlock::Single(Spanned::new(
                Op::PushString(st.clone()),
                statement.address.clone(),
            ))),
            ExprKind::Assignment { name, value } => {
                let compiled_expr = self.compile_expr(value)?.into_content();

                let mut result = vec![];

                result.extend_from_slice(&compiled_expr);
                result.push(Spanned::new(
                    Op::Define(
                        name.value
                            .as_id()
                            .expect("Expected identifier as variable name.")
                            .into(),
                    ),
                    statement.address.clone(),
                ));

                // todo!("Transform assignment! Name: {name:?}; Value: {compiled_expr:?}");

                Ok(VMBlock::Block { code: result })
            }
            _ => todo!("Compile other expression kinds"),
        }

        // todo!("WHAT");

        // Ok()
    }

    fn compile_function(&self, func: &Function) -> Result<VMBlock, String> {
        let name = match &func.name.value {
            ExprKind::Identifier(id) => id,
            kind => todo!("Function name is complex: {kind:?}"),
        };

        let body = match &*func.body {
            Expr(spanned) => {
                match &spanned.value {
                    ExprKind::Block(bk) => bk,
                    _ => unreachable!("Function body is not a block expression")
                }
            },
            _ => unreachable!("Function body is not an expression"),
        };

        let body_compiled = self.compile(&body)?;

        todo!("Function: name: {name:?}; Body: {body_compiled:?}");
    }
}
