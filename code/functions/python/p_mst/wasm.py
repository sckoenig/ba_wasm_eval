import sys
from unittest.mock import Mock

import bench

# gzip and zlib are not available in componentize-py environment
# for more see: https://github.com/bytecodealliance/componentize-py/issues/96
# and: https://opensource.snarky.ca/Python/WASI#Plans
sys.modules['bz2'] = Mock()
sys.modules['zlib'] = Mock()

from p_mst import main


# Wasm specific implementation for the benchmark using WASI:
# Implements the WASI run-Interface to be called by Wasmtime CLI.
class Run:
    def run(self) -> None:
        bench.benchmark(main)
