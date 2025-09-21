from bench import benchmark

from p_prime import main


# Wasm specific implementation for the benchmark using WASI:
# Implements the WASI run-Interface to be called by Wasmtime CLI.
class Run:
    def run(self) -> None:
        benchmark(main)
