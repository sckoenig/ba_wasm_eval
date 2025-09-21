/**
 * Implements a prime benchmark. Calculates the first n prime numbers
 * and printing them to stdout.
 *
 * Inspired by:
 * Vojdan Kjorveziroski & Sonja Filiposka 2023
 * https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/matmul/main.go
 */
export function main(n) {
    const res = prime(n);
    console.log(res.slice(0, 200));
}

/**
 * Calculates the first n prime numbers using the Sieve of Eratosthenes.
 * @param n inclusive upper bound
 * @returns {any} list of prime numbers <= n
 */
function prime(n) {
    const isPrime = new Array(n + 1).fill(true);
    isPrime[0] = false;
    if (n >= 1) {
        isPrime[1] = false;
    }

    const limit = Math.floor(Math.sqrt(n));
    for (let i = 2; i <= limit; i++) {
        if (isPrime[i]) {
            for (let j = i * i; j <= n; j += i) {
                isPrime[j] = false;
            }
        }
    }
    return isPrime.reduce((primes, primeFlag, i) => {
        if (primeFlag) {
            primes.push(i);
        }
        return primes;
    }, []);
}
