import {benchmark} from "../bench/bench.js";
import {main} from "./float.js";

/**
 * Main Code executing the benchmark.
 */
benchmark(() => {
    const args = process.argv.slice(2);
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