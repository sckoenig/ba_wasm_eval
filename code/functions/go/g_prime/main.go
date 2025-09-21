// Implements a prime benchmark, calculating the
// first n prime numbers using the Sieve of Eratosthenes and
// printing them to stdout.

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
			_, _ = fmt.Fprintf(os.Stderr, "Usage: g_prime LIMIT")
			return
		}
		n, err := strconv.Atoi(os.Args[1])
		if err != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			return
		}
		result := prime(n)
		count := len(result)
		fmt.Printf("Found: %d: %d ... ", count, result[:min(200, count)])
	})
}

// prime calculates all prime numbers <= n using the Sieve of Eratosthenes.
func prime(n int) []int {
	isPrime := make([]bool, n+1)
	var primes []int

	for i := 2; i <= n; i++ {
		isPrime[i] = true
	}

	sqrtN := int(math.Sqrt(float64(n)))
	for i := 2; i <= sqrtN; i++ {
		if isPrime[i] {
			for j := i * i; j <= n; j += i {
				isPrime[j] = false
			}
		}
	}
	for i := 2; i <= n; i++ {
		if isPrime[i] {
			primes = append(primes, i)
		}
	}
	return primes
}
