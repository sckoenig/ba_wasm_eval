// Package bench provides a minimal benchmark harness to measure function performance via timestamps.
package bench

import (
	"fmt"
	"os"
	"time"
)

// Timestamp returns the current time in microseconds since the Unix epoch.
func Timestamp() int64 {
	return time.Now().UnixMicro()
}

// Benchmark runs the provided function, recording timestamps before and after execution
// and logging them to stderr in the format: <start> <end>
func Benchmark(function func()) {
	start := Timestamp()
	function()
	end := Timestamp()
	_, _ = fmt.Fprintf(os.Stderr, "%d %d\n", start, end)
}
