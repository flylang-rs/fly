use dumpster::sync::CollectInfo;

pub fn gc_collect_condition(info: &CollectInfo) -> bool {
    let dropped = info.n_gcs_dropped_since_last_collect();
    let yeah = dropped.is_multiple_of(4096);

    // if yeah {
    //     eprintln!("GC: Collect!");
    // }

    yeah
}

/// This structure should be used as a field inside of `Interpreter`, a drop implementation will
/// trigger it and then Dumpster GC collection will be triggered.
#[derive(Clone)]
pub struct DumpsterGCHandle {}

impl Default for DumpsterGCHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl DumpsterGCHandle {
    pub fn new() -> Self {
        dumpster::sync::set_collect_condition(gc_collect_condition);

        DumpsterGCHandle {}
    }
}

impl Drop for DumpsterGCHandle {
    fn drop(&mut self) {
        dumpster::sync::collect();
    }
}

