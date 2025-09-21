/**
 * Provides a minimal benchmark harness to measure function performance via timestamps using WASI APIs.
 */
import {now} from 'wasi:clocks/wall-clock@0.2.0'

/**
 * Returns the current timestamp in microseconds since the Unix epoch (Unix time).
 * @returns {bigint} Current timestamp in microseconds.
 * Uses the WASI API for system clock access.
 */
export function wasm_timestamp() {
    //    WASI: record datetime {
    //         seconds: u64,
    //         nanoseconds: u32,
    //     }
    const timestamp = now();
    return timestamp.seconds * 1000000n + BigInt(Math.round(timestamp.nanoseconds / 1000)); // to microseconds
}

/**
 * Runs the given function, recording timestamps before and after execution,
 * and logs them to stderr.
 * @param {Function} func - The function to benchmark.
 */
export function wasm_benchmark(func) {
    const start = wasm_timestamp();
    func();
    const end = wasm_timestamp();
    console.error(`${start} ${end}`);
}
