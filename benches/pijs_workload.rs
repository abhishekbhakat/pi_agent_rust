//! `PiJS` workload bench target. The implementation is shared with the
//! `pijs_workload` example (examples/pijs_workload/); this crate root exists
//! so `cargo bench --bench pijs_workload` compiles the same harness without
//! registering one source path under two build targets.
#![recursion_limit = "256"]
#![forbid(unsafe_code)]

#[path = "../examples/pijs_workload/workload.rs"]
mod workload;

fn main() {
    workload::main();
}
