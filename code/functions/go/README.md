### Go Benchmark Functions

---
### Contents
- Benchmark functions executing the benchmark via their `main`-function
- `bench` containing basic benchmarking logic
  used by the functions
- `Dockerfile` is a template Dockerfile that can be used for every function by setting the FUNCTION argument.
- `build_go.sh` can be used to build Go artifacts: 
  ````
  # to build all artifacts:
  $ ./build_go.sh

  # to build a single function in a specific mode, e.g.:
  $ ./build_go.sh -m wasm -f g_prime
  ````

---
### Go to WebAssembly 
To build Go based WebAssembly components, the [TinyGo](https://tinygo.org/) compiler is used:
```
$ GOOS=wasip2 GOARCH=wasm tinygo build -o <function>.wasm main.go
```


