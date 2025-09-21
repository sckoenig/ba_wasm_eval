import aesjs from "aes-js";

/**
 * Implements an aes benchmark.
 * First creates a random string of a given size and
 * then encrypts and decrypts the string a given number of times.
 *
 * Inspired by:
 * XFBench: A Cross-Cloud Benchmark Suite for Evaluating FaaS Workflow Platforms
 * https://github.com/dream-lab/XFBench/blob/CCGRID2024/functions/text/encrypt/encrypt.py
 * Vojdan Kjorveziroski & Sonja Filiposka 2023
 * https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/aes/main.go
 */
export function main(n) {
    const msg = generateRandomString(n)
    encryptAndDecrypt(msg, n)
}

/**
 * Encrypts and decrypts the given string for a given number of times.
 */
function encryptAndDecrypt(text, iterations) {

    let msg_bytes = aesjs.utils.utf8.toBytes(text)

    for (let i = 0; i < iterations; i++) {

        let cipher = new aesjs.ModeOfOperation.ctr(KEY);
        const encryptedBytes = cipher.encrypt(msg_bytes);

        cipher = new aesjs.ModeOfOperation.ctr(KEY);
        msg_bytes = cipher.decrypt(encryptedBytes)
    }
    const decrypted = aesjs.utils.utf8.fromBytes(msg_bytes);
    console.log(`decrypted == original text? ${decrypted === text}`);
}

const ALPHABET = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456789?!*_:;!%&/()=?';
const KEY = new Uint8Array([0xa1, 0xf6, 0x25, 0x8c, 0x87, 0x7d, 0x5f, 0xcd, 0x89, 0x64, 0x48, 0x45, 0x38, 0xbf, 0xc9, 0x2c]);

function generateRandomString(length) {
    const result = new Array(length);
    const alphaLen = ALPHABET.length;

    for (let i = 0; i < length; i++) {
        result[i] = ALPHABET.charAt(Math.floor(Math.random() * alphaLen));
    }
    return result.join('')
}
