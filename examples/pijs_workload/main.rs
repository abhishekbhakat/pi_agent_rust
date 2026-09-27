//! `PiJS` workload harness for deterministic perf baselines.
//! Crate root for the `pijs_workload` example target; the implementation
//! lives in `workload.rs`, shared with the `pijs_workload` bench target.
#![recursion_limit = "256"]
#![forbid(unsafe_code)]

mod workload;

fn main() {
    workload::main();
}
