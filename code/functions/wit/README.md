### .wit Files

---

This directory contains the .wit files for the WASI `command` world in the 0.2 version.

The `command` world itself exports the `run`-Interface, which Wasmtime will
use to execute a component.
It also specifies WASI related imports a `command`-component needs
to interact with the system:

```` 
/// command.wit
package wasi:cli@0.2.0;

world command { 
  include imports; /// Includes the "imports" world, which contains WASI related imports

  export run; /// Exports the run function which is invokes by the wasm host
}
````

Taken from: https://github.com/bytecodealliance/componentize-py/tree/main/wit \
Up to date WASI Version can be found here: https://github.com/WebAssembly/WASI