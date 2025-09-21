//! Implements an aes benchmark.
//! First creates a random string of a given size and
//! then encrypts and decrypts the string a given number of times.
//!
//! Inspired by:
//! XFBench: A Cross-Cloud Benchmark Suite for Evaluating FaaS Workflow Platforms
//! https://github.com/dream-lab/XFBench/blob/CCGRID2024/functions/text/encrypt/encrypt.py
//! Vojdan Kjorveziroski & Sonja Filiposka 2023
//! https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/aes/main.go

use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes128Gcm, KeyInit};
use bench::*;
use rand::distr::{Alphanumeric, SampleString};
use std::env;

fn main() {
    benchmark(|| {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 1 {
            eprintln!("Usage: r_aes ITERATIONS");
            return;
        }
        match args[0].parse() {
            Err(e) => eprintln!("Error: {}", e),
            Ok(n) => {
                let string = Alphanumeric.sample_string(&mut rand::rng(), n);
                encrypt_and_decrypt(string, n);
            }
        }
    });
}

/// Encrypts and decrypts the given string for a given number of times.
fn encrypt_and_decrypt(text: String, iterations: usize) {
    let key = Aes128Gcm::generate_key(OsRng);
    let cipher = Aes128Gcm::new(&key);
    let mut msg_bytes: Vec<u8> =  text.as_bytes().to_vec();

    for _ in 0..iterations {
        let nonce = Aes128Gcm::generate_nonce(OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, msg_bytes.as_ref())
            .expect("encryption failed");

        let decrypted = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .expect("decryption failed");
        msg_bytes = decrypted;
    }
    let decrypted_str = String::from_utf8(msg_bytes).expect("invalid utf8");
    println!("decrypted == original text? {}", decrypted_str == text);
}
