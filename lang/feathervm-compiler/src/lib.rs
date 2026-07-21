use feathervm_definitions::{block::{BlockValue, VMBlock}, bytecode::Operation};
use flylang_parser::{
    ast::{
        ExprKind, Expression,
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
            Statement::Function(function) => todo!(),
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
                
                result.push(BlockValue::Add);

                Ok(VMBlock::Block { code: result })
            }
            ExprKind::Mul(a, b) => {
                let value_a = self.load_value(a)?;
                let value_b = self.load_value(b)?;

                let mut result = vec![];

                result.extend_from_slice(&value_a);
                result.extend_from_slice(&value_b);

                result.push(BlockValue::Mul);

                Ok(VMBlock::Block { code: result })
            }
            ExprKind::Number(nr) => {
                Ok(VMBlock::Single(BlockValue::PushNumber(nr.clone())))
            }
            _ => todo!("Compile other expression kinds"),
        }

        // todo!("WHAT");

        // Ok()
    }
}
