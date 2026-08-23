use feathervm_definitions::block::{BlockValue, Closure, Op, VMBlock};
use flylang_common::spanned::Spanned;
use flylang_parser::{
    ast::{
        DivisionKind, ExprKind, Expression, Function, Statement, StatementKind,
    }, state,
};
use log::debug;

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
        // AST shouldn't be empty, so we can work safely with address info.
        assert!(!ast.is_empty());

        // Placeholder for the actual compilation logic.
        // In a real implementation, this would involve parsing the source code,
        // generating bytecode, and returning it as a vector of bytes.
        let mut blocks = vec![];

        for i in ast {
            let block = self.compile_statement(i)?;

            blocks.push(block);
        }

        // The code below is dedicated to the single Return opcode.
        let return_address = {
            let first = ast.first().map(|x| x.address.clone()).unwrap();
            let last = ast.last().map(|x| &x.address).unwrap();

            first.merge(last)
        };

        blocks.push(VMBlock::Single(BlockValue::new(Op::Return, return_address)));

        Ok(blocks)
    }

    fn compile_statement(&self, statement: &Statement) -> Result<VMBlock, String> {
        match &statement.value {
            StatementKind::Expr(spanned) => self.compile_expr(spanned),
            StatementKind::Break => todo!(),
            StatementKind::Continue => todo!(),
            StatementKind::VariableDefinition(variable_definition) => todo!(),
            StatementKind::Function(function) => self.compile_function(Spanned::new(function, statement.address.clone())),
            StatementKind::If(_) => todo!(),
            StatementKind::While(_) => todo!(),
            StatementKind::RecordDefinition(record_definition) => todo!(),
            StatementKind::ModuleUsageDeclaration { path } => todo!(),
            StatementKind::Scope { held_value, body } => todo!(),
            StatementKind::Return { value } => todo!(),
            StatementKind::NoOp => self.compile_noop(statement),
        }
    }

    fn load_value(&self, expr: &Expression) -> Result<Vec<BlockValue>, String> {
        let value = self.compile_expr(expr)?;

        match value {
            VMBlock::Block { code } => Ok(code),
            VMBlock::Single(block_value) => Ok(vec![block_value]),
            VMBlock::Empty => Ok(vec![]),
        }
    }

    fn compile_noop(&self, _stmt: &Statement) -> Result<VMBlock, String> {
        Ok(VMBlock::Empty)
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
            ExprKind::Identifier(id) => Ok(VMBlock::Single(Spanned::new(
                Op::LoadName(id.to_string()),
                statement.address.clone()
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
            kind => todo!("Compile other expression kinds: {kind:?}"),
        }

        // todo!("WHAT");

        // Ok()
    }

    fn compile_function(&self, func: Spanned<&Function>) -> Result<VMBlock, String> {
        let (func, addr) = (func.value, func.address);

        let name = match &func.name.value {
            ExprKind::Identifier(id) => id,
            kind => todo!("Function name is complex: {kind:?}"),
        };

        let body = match &*&func.body.value {
            StatementKind::Expr(spanned) => match &spanned.value {
                ExprKind::Block(bk) => bk,
                _ => unreachable!("Function body is not a block expression"),
            },
            _ => unreachable!("Function body is not an expression"),
        };

        let arglist: Vec<_> = func
            .arguments
            .iter()
            .map(|x| {
                match x.value.as_id() {
                    Some(x) => x.to_string(),
                    None => panic!("Expected identifier as argument, got: {:?}", x)
                }
            }).collect();

        debug!("Argument list: {arglist:?}");

        let body_compiled = self.compile(&body)?;


        let closure = Op::Closure(Closure { body: body_compiled, arguments: arglist });

        let mut result = vec![];

        result.push(BlockValue::new(closure, addr.clone()));
        result.push(BlockValue::new(Op::Define(name.clone()), addr));

        // todo!("Function: name: {name:?}; Ops: {result:#?}");

        Ok(VMBlock::Block { code: result })
    }
}
