### A minimal WebAssembly Host
This repository contains code for a minimal WebAssembly host,
that embeds the Wasmtime runtime.

It runs components implementing the `command`-world
using the exported `run`-function

```
package wasi:cli@0.2.0;

world command {
  include imports; /// WASI related imports

  export run; /// executed by Host
}

```

Read more about Wasmtime here: https://github.com/bytecodealliance/wasmtime \
Read more about the Component Model here: https://component-model.bytecodealliance.org/ \
Read more about WASI here: https://github.com/WebAssembly/WASI