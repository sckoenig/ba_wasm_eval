/**
 * Implements a matrix multiplication benchmark.
 * Two quadratic matrices are first generated pseudorandomly and then multiplied
 * and the result printed to stdout.
 *
 * Inspired by:
 * Vojdan Kjorveziroski & Sonja Filiposka 2023
 * https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/rust/prime-numbers/src/main.rs
 */
export function main(n) {
    const a = generateMatrix(n, SEED1)
    const b = generateMatrix(n, SEED2)
    const c = matmul(a, b)
    console.log(`Res: ${c.slice(0,1)} ...`)
}

/**
 * Generates a quadratic matrix of dimension dim x dim filled with
 * pseudorandom floats.
 * @param dim matrix dimension
 * @param seed seed for rng
 * @returns {any[][]} dim x dim matrix
 */
function generateMatrix(dim, seed) {
    const rng = new LCG(seed);
    const matrix = Array.from({length: dim}, () => new Array(dim).fill(0));
    for (let i = 0; i < dim; i++) {
        for (let j = 0; j < dim; j++) {
            matrix[i][j] = rng.nextF64();
        }
    }
    return matrix;
}

/**
 * Multiplies two matrices, creating a new matrix.
 * @param a Matrix a(n x m)
 * @param b Matrix b(m x p),
 * @returns {any[][]} Matrix c(n x p)
 */
function matmul(a, b) {
    const n = a.length;
    const m = b.length;
    const p = b[0].length;
    const c = Array.from({length: n}, () => new Array(p).fill(0));

    for (let i = 0; i < n; i++) {
        for (let j = 0; j < p; j++) {
            for (let k = 0; k < m; k++) {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    return c;
}

const SEED1 = 32;
const SEED2 = 64;
const M = 1n << 48n; // 2^48
const C = 11n;
const A = 25214903917n;

/**
 * Linear congruential generator for pseudorandom number generation.
 * https://en.wikipedia.org/wiki/Linear_congruential_generator
 */
class LCG {
    constructor(seed) {
        this.seed = BigInt(seed);
    }

    /**
     * Returns the next pseudorandom float using Xn+1 = (A * Xn + C) % M
     */
    nextF64() {
        this.seed = (A * this.seed + C) % M;
        return Number(this.seed) / Number(M);
    }
}