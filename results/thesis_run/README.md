## Results

This directory contains all the raw data used in the thesis evaluation.
All measurements were made on Ubuntu, using the following system and software versions:

| Component        | Description                            |
| ---------------- | -------------------------------------- |
| Operating System | Ubuntu 24.04.2 LTS                     |
| CPU              | AMD Ryzen 7 2700X Eight‑Core Processor |
| RAM              | 32 GiB DDR4                            |
| Storage          | Samsung SSD 870 EVO 500 GB SATA        |


| Software                                                               | Version |
|------------------------------------------------------------------------| ------- |
| [Rust](https://www.rust-lang.org/)                                     | 1.85.1  |
| [Python](https://www.python.org/downloads/)                            | 3.12.3  |
| [Node.js](https://nodejs.org/en)                                       | 22.14.0 |
| [Go](https://go.dev/doc/install)                                       | 1.24.3  |
| [Docker](https://www.docker.com/)                                      | 28.0.4  |
| [Wasmtime CLI](https://wasmtime.dev/)                                  | 33.0.0  |
| [componentize-py](https://github.com/bytecodealliance/componentize-py) | 0.17.1  |
| [ComponentizeJS](https://github.com/bytecodealliance/ComponentizeJS)   | 0.18.2  |
| [TinyGo](https://tinygo.org/)                                          | 0.37.0  |
| [wasm-tools](https://github.com/bytecodealliance/wasm-tools)           | 1.228.0 |

| Library              | Language   | Used In          | Version |
| -------------------- | ---------- | ---------------- | ------- |
| wasmtime             | Rust       | WebAssembly Host | 33.0.0  |
| wasmtime‑wasi        | Rust       | WebAssembly Host | 33.0.0  |
| image                | Rust       | thumbnailer      | 0.25.6  |
| askama               | Rust       | template         | 0.14.0  |
| random‑number        | Rust       | template         | 0.1.9   |
| petgraph             | Rust       | mst              | 0.8.1   |
| rand                 | Rust       | mst, aes         | 0.9.1   |
| aes‑gcm              | Rust       | aes              | 0.10.3  |
| anthonynsimon/bild   | Go         | thumbnailer      | 0.14.0  |
| flosch/pongo2        | Go         | template         | 6.0.0   |
| dominikbraun/graph   | Go         | mst              | 0.23.0  |
| xyproto/randomstring | Go         | aes              | 1.2.0   |
| Jinja2               | Python     | template         | 3.1.6   |
| networkx             | Python     | mst              | 3.4.2   |
| pyaes                | Python     | aes              | 1.6.1   |
| mustache             | JavaScript | template         | 4.2.0   |
| js‑graph‑algorithms  | JavaScript | mst              | 1.0.18  |
| aes‑js               | JavaScript | aes              | 3.1.2   |
