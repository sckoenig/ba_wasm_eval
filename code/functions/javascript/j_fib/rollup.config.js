import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';

const entryMap = {
    main: './main.js',
    wasm: './wasm.js'
};
const entryName = process.env.ENTRY || 'main';

export default {
    input:  entryMap[entryName],
    output: {
        file: `bundle/${entryName}_bundle.mjs`,
        format: 'esm',
        sourcemap: true
    },
    plugins: [
        nodeResolve({ browser: true }),
        commonjs()
    ]
};