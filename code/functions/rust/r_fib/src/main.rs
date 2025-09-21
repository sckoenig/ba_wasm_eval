//! Implements a fibonacci benchmark,
//! calculating the nth fibonacci number recursively.

use bench::benchmark;
use std::env;

fn main() {
    benchmark(|| {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 1 {
            eprintln!("Usage: r_fib NTH");
            return;
        }

        match args[0].parse() {
            Err(e) => eprintln!("Error: {}", e),
            Ok(n) => {
                let res = fibonacci(n);
                println!("{res}");
            }
        }
    })
}

fn fibonacci(n: usize) -> usize {
    if n <= 1{
        return n
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
