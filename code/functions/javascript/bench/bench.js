/**
 * Provides a minimal benchmark harness to measure function performance via timestamps.
 */

/**
 * Returns the current timestamp in microseconds since the Unix epoch (Unix time).
 * @returns {number} Current timestamp in microseconds.
 */
export function timestamp() {
    const t = performance.timeOrigin + performance.now(); //milliseconds
    return Math.round(t * 1000); // to micro
}

/**
 * Runs the given function, recording timestamps before and after execution
 * and logging them to stderr.
 * @param {Function} func - The function to benchmark.
 */
export function benchmark(func) {
    const start = timestamp();
    func();
    const end = timestamp();
    console.error(`${start} ${end}`);
}