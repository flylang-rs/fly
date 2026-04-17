use std::sync::Arc;

use flylang_common::source::Source;
use flylang_diagnostics::error::DiagnosticsReport;
use flylang_lexparse_glue::LoadingError;

use crate::{control_flow::ControlFlow, error::InterpreterError};

pub type TestResult<T> = Result<T, TestError>;

#[derive(Debug)]
pub enum TestError {
    LoadingError(LoadingError),
    InterpreterError(InterpreterError),
}

impl DiagnosticsReport for TestError {
    fn render(&self) -> String {
        match self {
            TestError::LoadingError(loading_error) => loading_error.render(),
            TestError::InterpreterError(interpreter_error) => interpreter_error.render(),
        }
    }
}

fn execute_inner(source: Source) -> TestResult<ControlFlow> {
    let source = Arc::new(source);

    let ast = flylang_lexparse_glue::parse_source(Arc::clone(&source))
        .map_err(TestError::LoadingError)?;

    let mut interpreter = crate::Interpreter::new();

    interpreter.execute(ast).map_err(TestError::InterpreterError)
}

pub fn execute(code: &str) -> TestResult<ControlFlow> {
    execute_inner(Source::new("<test>".to_string(), code.to_string()))
}


pub fn execute_or_fail(code: &str) -> ControlFlow {
    let value = execute_inner(Source::new("<test>".to_string(), code.to_string()));

    match value {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{}", e.render());

            panic!("Encountered an error.")
        }
    }
}