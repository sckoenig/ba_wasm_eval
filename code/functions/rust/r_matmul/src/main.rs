//! Implements a matrix multiplication benchmark.
//! Two quadratic matrices are first generated pseudorandomly and then multiplied
//! and the result printed to stdout.
//!
//! Inspired by:
//! Vojdan Kjorveziroski & Sonja Filiposka 2023
//! https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/matmul/main.go

use bench::benchmark;
use std::env;

fn main() {
    benchmark(|| {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 1 {
            eprintln!("Usage: r_matmul DIMENSION");
            return;
        }
        match args[0].parse() {
            Err(e) => eprintln!("Error: {}", e),
            Ok(dim) => {
                let a = generate_matrix(dim, SEED1);
                let b = generate_matrix(dim, SEED2);
                let c = mat_mul(&a, &b);
                println!("Res: {:?} ...", &c[..1]);
            }
        }
    });
}

/// Multiplies two matrices a(n x m) and Matrix b(m x p), creating a new matrix c(n x p).
fn mat_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let (n, m, p) = (a.len(), b.len(), b[0].len());
    let mut c = vec![vec![0.0; p]; n];

    for i in 0..n {
        for j in 0..p {
            for k in 0..m {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

const SEED1: u128 = 32;
const SEED2: u128 = 64;
const M: u128 = 1 << 48;
const C: u128 = 11;
const A: u128 = 25214903917;

/// Linear congruential generator for pseudorandom number generation.
/// https://en.wikipedia.org/wiki/Linear_congruential_generator
struct Lcg {
    seed: u128,
}

impl Lcg {
    /// Returns the next pseudorandom float using Xn+1 = (A * Xn + C) % M
    fn next_f64(&mut self) -> f64 {
        self.seed = (A * self.seed + C) % M;
        self.seed as f64 / (M as f64)
    }
}

/// Generates a quadratic matrix filled with pseudorandom floats.
fn generate_matrix(dim: usize, seed: u128) -> Vec<Vec<f64>> {
    let mut rng = Lcg { seed };
    let mut matrix = vec![vec![0.0; dim]; dim];
    for i in 0..dim {
        for j in 0..dim {
            matrix[i][j] = rng.next_f64();
        }
    }
    matrix
}
