pub trait DiagnosticsReport {
    fn render(&self) -> String;
}