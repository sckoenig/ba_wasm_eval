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
	"math"
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
		result := floatOps(n)
		fmt.Println(result)
	})
}

// floatOps computes sin, cos, and sqrt over [0,n) and accumulates the result.
func floatOps(n int) float64 {
	sum := 0.0
	for i := 0; i < n; i++ {
		f := float64(i)
		sum += math.Sin(f)
		sum += math.Cos(f)
		sum += math.Sqrt(f)
	}
	return sum
}
