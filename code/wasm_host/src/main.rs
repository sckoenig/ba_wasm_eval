//! This is a minimal Host for WebAssembly command components with a WASI context.
//! It makes a timestamp right before calling a component.
//! It can either read precompiled components (.cwasm) from file or jit compile a component (.wasm).

use wasmtime_wasi::p2::add_to_linker_sync;
use anyhow::ensure;
use std::time::{SystemTime, UNIX_EPOCH};
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::*;
use wasmtime_wasi::{DirPerms, FilePerms};

use clap::Parser;
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi::p2::bindings::sync::Command;

#[derive(Parser)]
struct Args {
    path: String,
    #[arg(long, default_value_t = false)]
    aot: bool,
    wasm_args: Vec<String>,
}

fn main() -> Result<()> {
    // read args
    let args = Args::parse();
    println!(
        "Processing {}, AOT: {}, Args: {:?}",
        args.path, args.aot, args.wasm_args
    );

    let invoker = Invoker::new()?;

    eprint!("{} ", timestamp());
    invoker.invoke_wasm(args.path, args.aot, args.wasm_args)?;

    Ok(())
}

pub struct Invoker {
    /// compilation and runtime environment for wasm, one per process
    engine: Engine,
    /// instantiating components and linking host functionality, reused for every instantiation
    linker: Linker<ComponentStates>,
}

impl Invoker {
    pub fn new() -> Result<Self> {
        let engine = Engine::default();
        let mut linker: Linker<ComponentStates> = Linker::new(&engine);
        add_to_linker_sync(&mut linker)?;

        Ok(Invoker { engine, linker })
    }

    /// Instantiates the given wasm component with the given args and invokes
    /// its run function.
    fn invoke_wasm(&self, path: String, aot: bool, mut wasm_args: Vec<String>) -> Result<()> {
        // build wasi context for component
        wasm_args.insert(0, String::new());

        let wasi_ctx = WasiCtxBuilder::new()
            // access to host's stdin, stdout and stderr
            .inherit_stdio()
            // make host directory "." available as "/" in wasm guest with permissions
            .preopened_dir("./input", "/input", DirPerms::all(), FilePerms::all())?
            // list of program arguments wasm has access to
            .args(&wasm_args)
            .build();

        let state = ComponentStates {
            wasi_ctx,
            resource_table: ResourceTable::new(),
        };
        let mut store = Store::new(&self.engine, state);

        // load component from file
        // reading aot must be marked as unsafe as the file is not validated to be a valid, precompiled module
        let component = match aot {
            true => unsafe { Component::deserialize_file(&self.engine, path)? },
            false => Component::from_file(&self.engine, path)?,
        };

        // execute
        let command = Command::instantiate(&mut store, &component, &self.linker)?;
        let exit = command.wasi_cli_run().call_run(&mut store)?;
        ensure!(exit.is_ok(), "WASI program exited with non-zero exit code");

        Ok(())
    }
}

/// States required by a component.
pub struct ComponentStates {
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}
impl IoView for ComponentStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}
impl WasiView for ComponentStates {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros()
}
