//! Implements a prime benchmark, calculating the
//! first n prime numbers using the Sieve of Eratosthenes and
//! printing them to stdout.

use std::cmp::min;
use bench::benchmark;
use std::env;

fn main() {
    benchmark(|| {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 1 {
            eprintln!("Usage: r_prime LIMIT");
            return;
        }

        match args[0].parse() {
            Err(e) => eprintln!("Error: {}", e),
            Ok(n) => {
                let res = prime(n);
                println!("{:?} ...", &res[..min(200, res.len())]);
            }
        }
    })
}

/// Calculates the first n prime numbers using the Sieve of Eratosthenes.
/// # Arguments
/// *`n` (inclusive) for which to compute primes
/// # Returns
/// Vector containing all prime numbers <= n.
fn prime(n: usize) -> Vec<usize> {
    let mut is_prime: Vec<bool> = vec![true; n + 1];

    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    let sqrt = usize::isqrt(n);
    for i in 2..=sqrt{
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    (2..=n).filter(|&i| is_prime[i]).collect()
}
