/// This structure should be used as a field inside of `Interpreter`, a drop implementation will
/// trigger it and then Dumpster GC collection will be triggered.
pub struct DumpsterGCDropTrigger;

impl Drop for DumpsterGCDropTrigger {
    fn drop(&mut self) {
        dumpster::sync::collect();
    }
}

