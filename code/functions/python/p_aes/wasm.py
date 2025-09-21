import bench

from p_aes import main


# Wasm specific implementation for the benchmark using WASI:
# Implements the WASI run-Interface to be called by Wasmtime CLI.
class Run:
    def run(self) -> None:
        bench.benchmark(main)
