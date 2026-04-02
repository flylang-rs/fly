pub enum DiagnosticsKind {
    Error,
    Warning,
}

impl DiagnosticsKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticsKind::Error => "error",
            DiagnosticsKind::Warning => "warning",
        }
    }

    pub fn color(&self) -> owo_colors::AnsiColors {
        match self {
            DiagnosticsKind::Error => owo_colors::AnsiColors::Red,
            DiagnosticsKind::Warning => owo_colors::AnsiColors::Yellow,
        }
    }
}
