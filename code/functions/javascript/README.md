### JavaScript Benchmark Functions

---
### Contents
- `bench` implements basic benchmarking logic and is being used by every function. 
- `Dockerfile` is a template Dockerfile that can be used for every function by setting the FUNCTION argument.
- `build_js.sh` can be used to build JavaScript artifacts:
  ````
  # to build all artifacts:
  $ ./build_js.sh

  # to build a single function in a specific mode, e.g.:
  $ ./build_js.sh -m wasm -f j_prime
  ````


`bench` has two different implementations. The `bench.js` is for local and Docker execution,
while the `bench_wasm.js` is used for wasm. The reason is, that
the behavior of `performance.timeOrigin` differs between execution in Node.js and Wasmtime.
The wasm implementation uses the WASI clock API which provides similar behavior to 
`performance.timeOrigin` in node:

```
// import WASI clock functionality
import { now } from 'wasi:clocks/wall-clock@0.2.0'

/**
 * Returns the current timestamp in nanoseconds since the Unix epoch (Unix time).
 * @returns {bigint} Current timestamp in nanoseconds.
 * Uses the WASI API for system clock access.
 */
export function wasm_timestamp() {
    const timestamp = now();
    return (timestamp.seconds * 1000000000n) + BigInt(timestamp.nanoseconds);
}
```

For each benchmark functions, there are these files:
- config files for bundling
- **`<function>.js`**: Implements the function's logic.
- **`main.js`**: Runs the benchmark function.
- **`wasm.js`**: Implements the logic needed for a wasm component
  targeting the `command` world. For this, each Benchmark function must implement the `Run`-Interface.
  On execution, Wasmtime will invoke the `run()`-function. It also uses the WASI API to access program
  arguments, as `process.argv` is not available when using ComponentizeJS:

  ````
  // Example: matmul
  
  import { wasm_benchmark } from "../bench/wasm_bench.js";
  import { main } from "./matmul.js";
  import * as env from "wasi:cli/environment@0.2.0"; // WASI API
  
  /**
   * Wasm specific implementation for the benchmark using WASI
   * Implements the WASI run-Interface to be called by Wasmtime CLI.
   */
  export const run = {
      run() {
          wasm_benchmark(() => {
              // calls benchmark using the WASI API to get program arguments from environment
              // as "process.argv" is not available using ComponentizeJS
              const args = env.getArguments().slice(1);
              //... run benchmark ...
          })
      }
  }
  ````


---
### JavaScript to WebAssembly via ComponentizeJS
To build JavaScript ESM based WebAssembly components, the Bytecode Alliance's 
[ComponentizeJS](https://github.com/bytecodealliance/ComponentizeJS) is used.

It takes the `world` being targeted and the associated `.wit` Files as an argument.
For easy execution the `command` world is targeted:
```
$ componentize-js -w path/to/wit -n wasi:cli/command -o <function>.wasm <function>.mjs
```

