### Python Benchmark Functions


---
### Contents
- `bench` implements basic benchmarking logic and is being used by every function. 
- `Dockerfile` is a template Dockerfile that can be used for every function by setting the FUNCTION argument.
- `build_py.sh` can be used to build Python artifacts:

  ````
  # to build all artifacts:
  $ ./build_py.sh

  # to build a single function in a specific mode, e.g.:
  $ ./build_py.sh -m wasm -f p_prime
  ````


For each benchmark functions, there are three files:
- **`<function>.py`**: Implements the function's logic.
- **`main.py`**: Runs the benchmark in its main-function for local and Docker execution.
- **`wasm.py`**: Implements the logic needed for a wasm component
  targeting the `command` world. For this, each Benchmark function must implement the `Run`-Interface.
  On execution, Wasmtime will invoke the `run()`-function:

  ````
  from bench import benchmark
  from p_prime import main

  # Wasm specific implementation for the benchmark using WASI:
  # Implements the WASI run-Interface to be called by Wasmtime CLI.
  class Run:
      def run(self) -> None:
          benchmark(main)
  ````

---
### Python to WebAssembly via Componentize-py

To build Python based WebAssembly components, the Bytecode Alliance's
[componentize-py](https://github.com/bytecodealliance/componentize-py) is used.

It takes the `world` being targeted and the associated `.wit` files as an argument. 
For easy execution the `command` world is targeted:
```
 $ componentize-py -d path/to/wit -w wasi:cli/command@0.2.0 componentize <function> -o <function>.wasm
```
