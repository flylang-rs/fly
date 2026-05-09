use dumpster::Trace;

#[derive(Debug, Copy, Clone, Trace)]
pub enum Visibility {
    Local,
    Global,
}
