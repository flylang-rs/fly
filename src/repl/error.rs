use flylang_diagnostics::error::DiagnosticsReport;
use flylang_lexparse_glue::LoadingError;
use flylang_tte::error::InterpreterError;

pub type REPLResult<T> = Result<T, REPLError>;

#[derive(Debug)]
pub enum REPLError {
    ModuleLoadingError(LoadingError),
    InterpreterError(InterpreterError),
}

impl DiagnosticsReport for REPLError {
    fn render(&self) -> String {
        match self {
            REPLError::ModuleLoadingError(loading_error) => loading_error.render(),
            REPLError::InterpreterError(interpreter_error) => interpreter_error.render(),
        }
    }
}