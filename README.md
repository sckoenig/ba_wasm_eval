## Evaluating WebAssembly for Efficient Function Isolation in Serverless Computing

----
### Introduction
This repository contains code and data used for the practical evaluation in the Bachelor thesis
"Evaluating WebAssembly for Efficient Function Isolation in Serverless Computing".
In this project, WebAssembly and Docker are compared as isolation techniques for serverless functions.
Using a mix of micro- and macro-benchmarks adapted from or inspired by existing scientific publications 
and serverless benchmark suites, the following metrics are evaluated:
- artifact sizes of ```wasm```, AOT-compiled ```cwasm``` and docker images
- cold start times 
- compute times
- response times

For this evaluation, the languages Rust, Go, Python and JavaScript are included,
as they appear on prominent WebAssembly based platforms such as 
[Spin](https://spinframework.dev/) / [Fermyon Cloud](https://www.fermyon.com/), 
[wasmCloud](https://wasmcloud.com/)
and [Fastly](https://www.fastly.com/) (as of June 2025).

----
### Content

The repository contains the following:

```` 
root/
├── code/
│   ├── functions/
│   │   ├── go/
│   │   ├── javascript/
│   │   ├── python/
│   │   ├── rust/
│   │   └── wit/
│   └── wasm_host/
├── input/
├── results/
│   └── thesis_run/
│       ├── artifact_sizes/
│       ├── diagrams/
│       └── performance/
├── scripts/
│   ├── build_all.sh
│   ├── run_bench.sh
│   └── run_function.sh
└── eval.ipynb

````
- ```code/functions```: Contains all function code organized by language as well as build scripts and dockerfiles for each language.
- ```code/functions/wit```: Contains the ```.wit``` files needed for working with WASI Preview 2 and the component model.
- ```code/wasm_host```: Contains code of the WebAssembly Host used in the evaluation. It embeds [Wasmtime](https://github.com/bytecodealliance/wasmtime), 
a standalone WebAssembly Runtime developed the Bytecode Alliance.
- ```input```: Contains the function input data used in the evaluation.
- ```results```: Contains raw data from the evaluation in form of unix time stamps per function, language, execution mode and input size as well as artifact sizes
- ```scripts/build_all.sh```: Builds WebAssembly (```wasm```, ```cwasm```), Docker Images and, if necessary, native binaries for all functions and languages. 
- ```scripts/run_bench.sh```: Runs the whole benchmark. It reads inputs from a file, and then sequentially executes each function for each execution mode and input size.
- ```scripts/run_function.sh```: Can be used to run a single function in a specific configuration.
- ```evaluation.ipynb```: Jupyter Notebook for analyzing the evaluation data.


---

### Functions
The included benchmark functions are listed below.
If a functions has been inspired by or adapted from another source, the source is directly mentioned in
the corresponding file.

| Function    | Description                                                                                                           | Category                                           | Languages                    |
| ----------- |-----------------------------------------------------------------------------------------------------------------------|----------------------------------------------------| ---------------------------- |
| float       | Performs various floating‑point operations (sin, cos, sqrt) *n* times, sums the results, and outputs the final value. | Compute‑intensive                                  | Rust, Go, Python, JavaScript |
| fib         | Recursively computes the *n*th Fibonacci number and outputs it.                                                       | Compute‑intensive, memory‑intensive (recursion)    | Rust, Go, Python, JavaScript |
| matmul      | Multiplies two pseudo‑random *n×n* matrices and outputs the resulting matrix.                                         | Compute‑intensive, memory access                   | Rust, Go, Python, JavaScript |
| prime       | Calculates the first *n* prime numbers using the Sieve of Eratosthenes and outputs them.                              | Compute‑intensive, memory‑intensive (large arrays) | Rust, Go, Python, JavaScript |
| aes         | Encrypts and decrypts a randomly generated string of length *n*, repeated *n* times, and outputs the final result.    | Encryption                                         | Rust, Go, Python, JavaScript |
| mst         | Computes the minimum spanning tree of a randomly generated graph of size *n* and outputs the edges.                   | Scientific computing                               | Rust, Go, Python, JavaScript |
| template    | Generates *n* random numbers and renders them into HTML code based on a template.                                     | Web applications                                   | Rust, Go, Python, JavaScript |
| thumbnailer | Creates a thumbnail version of an image and saves it as a new file.                                                   | Image processing                                   | Rust, Go                     |

---
### Installation
To build the artifacts and run the benchmark, you will need the following tools installed.
The versions listed are the ones used in the evaluation of the thesis. 
Please note that the WebAssembly Host uses the `wasmtime` Crate v33.0.0
and the Wasmtime CLI needs to be of the same version, because 
it is used to AOT-compile `.wasm` to `cwasm`.

| Software                                                               | Version |
|------------------------------------------------------------------------|---------|
| [Rust](https://www.rust-lang.org/)                                     | 1.85.1  |
| [Python](https://www.python.org/downloads/)                            | 3.12.3  |
| [Node.js](https://nodejs.org/en)                                       | 22.14.0 |
| [Go](https://go.dev/doc/install)                                       | 1.24.3  |
| [Docker](https://www.docker.com/)                                      | 28.0.4  |
| [Wasmtime CLI](https://wasmtime.dev/)                                  | 33.0.0  |
| [componentize-py](https://github.com/bytecodealliance/componentize-py) | 0.17.1  |
| [ComponentizeJS](https://github.com/bytecodealliance/ComponentizeJS)   | 0.18.2  |
| [TinyGo](https://tinygo.org/)                                          | 0.37.0  |
| [wasm-tools](https://github.com/bytecodealliance/wasm-tools)           |  1.228.0       |


To compile Rust to WebAssembly, simply download the `wasm32-wasip2` target:
```shell
rustup target add wasm32-wasip2
```


As of June 2025, you can install [componentize-py](https://github.com/bytecodealliance/componentize-py)
and [ComponentizeJS](https://github.com/bytecodealliance/ComponentizeJS) with:
```shell
pip install componentize-py==0.17.1
npm install -g @bytecodealliance/componentize-js@0.18.2
```
TinyGo needs `wasm-tools`, you can install it via cargo:
```shell
cargo install wasm-tools --version 1.228.0
```

---

### Running the benchmark

To run the benchmarks, follow these steps:
1. Clone the repository.
2. Make sure you have all necessary tools installed.
3. If necessary, adjust the input parameters.
4. Run the `build_all.sh` script to build all artifacts.
    ```shell
    ./scripts/build_all.sh
    ```
5. Run the `run_bench.sh` script to run the benchmark:
    ```shell
    ./scripts/run_bench.sh
    ```
    Per default, the script will run every function in every execution mode with every given input size for 25 iterations.
    You can adjust this using the `-l` (language), `-s` (input size), `-i` (iterations) and `m` (mode) flags.
    You can use another input file using the `-f` flag.
    
    E.g. run for 5 iterations on test.csv input file:
    ```shell
    ./scripts/run_bench.sh -i 1 -f ./input/test.csv
    ```
6. Use the jupyter notebook for results and visualization

You can also run a single function with a specific configuration using the `run_function.sh` script.
