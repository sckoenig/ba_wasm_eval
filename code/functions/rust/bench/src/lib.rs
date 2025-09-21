//! Provides a minimal benchmark harness to measure function performance via timestamps.

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current timestamp in microseconds since the Unix epoch (Unix time).
pub fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros()
}

/// Runs the given function, recording timestamps before and after execution
/// and logging them to stderr.
pub fn benchmark(function: fn()) {
    let start = timestamp();
    function();
    let end = timestamp();
    eprintln!("{} {}", start, end); // for logfile
}
