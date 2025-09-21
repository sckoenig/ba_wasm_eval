//! Implements a float operation benchmark,
//! calculating the sin, cos and square root of n numbers.
//!
//! Adapted from:
//! FunctionBench
//! https://github.com/ddps-lab/serverless-faas-workbench/blob/master/aws/cpu-memory/float_operation/lambda_function.py

use bench::benchmark;
use std::env;

fn main() {
    benchmark(|| {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 1 {
            eprintln!("Usage: r_float ITERATIONS");
            return;
        }

        match args[0].parse() {
            Err(e) => eprintln!("Error: {}", e),
            Ok(n) => {
                let res = float_ops(n);
                println!("{res}");
            }
        }
    })
}

fn float_ops(n: usize) -> f64 {
    let mut sum = 0.0;
    for i in 0..n {
        let f = i as f64;
        sum += f64::sin(f);
        sum += f64::cos(f);
        sum += f64::sqrt(f);
    }
    sum
}
