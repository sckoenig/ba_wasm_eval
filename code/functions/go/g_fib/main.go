// Implements a float operation benchmark,
// calculating the sin, cos and square root of n numbers.
//
// Adapted from:
// FunctionBench
// https://github.com/ddps-lab/serverless-faas-workbench/blob/master/aws/cpu-memory/float_operation/lambda_function.py
package main

import (
	"bench"
	"fmt"
	"os"
	"strconv"
)

func main() {
	bench.Benchmark(func() {
		if len(os.Args) != 2 {
			_, _ = fmt.Fprintf(os.Stderr, "Usage: g_float ITERATIONS")
			return
		}
		n, err := strconv.Atoi(os.Args[1])
		if err != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			return
		}
		result := fibonacci(n)
		fmt.Println(result)
	})
}

// floatOps computes sin, cos, and sqrt over [0,n) and accumulates the result.
func fibonacci(n int) int {
	if n <= 1 {
		return n
	}
	return fibonacci(n-1) + fibonacci(n-2)
}
