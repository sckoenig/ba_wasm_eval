import {wasm_benchmark} from "../bench/wasm_bench.js";
import * as env from "wasi:cli/environment@0.2.0";
import {main} from "./float.js";

/**
 * Wasm specific implementation for the benchmark using WASI:
 * Implements the WASI run-Interface to be called by Wasmtime CLI.
 */
export const run = {
    run() {
        wasm_benchmark(() => {
            // calls benchmark using the WASI API to get program arguments from environment
            // as "process.argv" is not available using ComponentizeJS
            const args = env.getArguments().slice(1);
            if (args.length < 1) {
                console.error("Pass a number as argument.");
                return;
            }
            const n = Number(args[0]);
            if (Number.isNaN(n)) {
                console.error(`Error: '${args[0]}' is not a valid number`);
                return;
            }
            main(n)
        })
    }
}
