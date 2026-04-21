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

pub struct Tester {
    interp: crate::Interpreter,
}

impl Tester {
    pub fn new() -> Self {
        Self {
            interp: crate::Interpreter::new(),
        }
    }

    pub fn exec(&mut self, code: &str) -> TestResult<ControlFlow> {
        let src = Source::new("<test>".to_string(), code.to_string());

        let ast = flylang_lexparse_glue::parse_source(src.into())
            .map_err(TestError::LoadingError)?;

        self.interp.execute(ast).map_err(TestError::InterpreterError)
    }
    
    pub fn exec_script(&mut self, code: &str) -> TestResult<ControlFlow> {
        let src = Source::new("<test>".to_string(), code.to_string());

        let ast = flylang_lexparse_glue::parse_source(src.into())
            .map_err(TestError::LoadingError)?;

        self.interp.execute_script(ast).map_err(TestError::InterpreterError)
    }
}

pub fn execute(code: &str) -> TestResult<ControlFlow> {
    Tester::new().exec(code)
}