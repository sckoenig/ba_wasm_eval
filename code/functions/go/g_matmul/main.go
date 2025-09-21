// Implements a matrix multiplication benchmark.
// Two quadratic matrices are first generated pseudorandomly and then multiplied
// and the result printed to stdout.
//
// Inspired by:
// Vojdan Kjorveziroski & Sonja Filiposka 2023
// https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/matmul/main.go
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
			_, _ = fmt.Fprintf(os.Stderr, "Usage: g_matmul DIMENSION")
			return
		}
		n, err := strconv.Atoi(os.Args[1])
		if err != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			return
		}
		a := generateMatrix(n, SEED1)
		b := generateMatrix(n, SEED2)
		c := matMul(a, b)
		fmt.Printf("Res: %v ... \n", c[:1])
	})
}

// matMul multiplies two matrices a(n x m) and Matrix b(m x p), creating a new matrix c(n x p).
func matMul(a, b [][]float64) [][]float64 {
	n, m := len(a), len(b)
	p := len(b[0])
	c := make([][]float64, n)

	for i := 0; i < n; i++ {
		c[i] = make([]float64, p)
		for j := 0; j < p; j++ {
			for k := 0; k < m; k++ {
				c[i][j] += a[i][k] * b[k][j]
			}
		}
	}
	return c
}

// generateMatrix generates a quadratic matrix filled with pseudorandom floats.
func generateMatrix(dim int, seed uint64) [][]float64 {
	rng := &Lcg{seed: seed}
	matrix := make([][]float64, dim)
	for i := 0; i < dim; i++ {
		matrix[i] = make([]float64, dim)
		for j := 0; j < dim; j++ {
			matrix[i][j] = rng.nextF64()
		}
	}
	return matrix
}

// Lcg Linear congruential generator for pseudorandom number generation.
// https://en.wikipedia.org/wiki/Linear_congruential_generator
type Lcg struct {
	seed uint64
}

// nextF64 Returns the next pseudorandom float using Xn+1 = (A * Xn + C) % M
func (l *Lcg) nextF64() float64 {
	l.seed = (A*l.seed + C) % M
	return float64(l.seed) / float64(M)
}

const (
	SEED1 = 32
	SEED2 = 64
	M     = 1 << 48
	C     = 11
	A     = 25214903917
)
