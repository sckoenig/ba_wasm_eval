### Rust Benchmark Functions

---
### Contents
- Benchmark functions: Each function is implemented as a binary crate, executing
  the benchmark via its `main`-function
- `bench` library crate contains basic benchmarking logic
  used by the functions
- `Dockerfile` is a template Dockerfile that can be used for every function by setting the FUNCTION argument.
- `build_rust.sh` can be used to build Rust artifacts: 
    ```` 
    # to build all artifacts:
    $ ./build_rust.sh
    
    # to build a single function in a specific mode, e.g.:
    $ ./build_rust.sh -m wasm -f r_prime
    ````

---
### Rust to WebAssembly 
To build Rust based WebAssembly components, the `wasm32-wasip2` target is used:
```
$ cargo build --release --target=wasm32-wasip2 -p <function>
```


