import os
import random
import sys

import pyaes


def main():
    """
    Implements an aes benchmark.
    First creates a random string of a given size and
    then encrypts and decrypts the string a given number of times.

    Adapted from:
    XFBench: A Cross-Cloud Benchmark Suite for Evaluating FaaS Workflow Platforms
    https://github.com/dream-lab/XFBench/blob/CCGRID2024/functions/text/encrypt/encrypt.py
    Vojdan Kjorveziroski & Sonja Filiposka 2023
    https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/aes/main.go
    """
    args = sys.argv[1:]
    if len(args) != 1:
        print("Pass a number as argument.", file=sys.stderr)
        return
    try:
        string_length = iterations = int(args[0])
    except ValueError:
        print("Pass a number as argument.", file=sys.stderr)
        return

    msg = generate_random_string(string_length)
    encrypt_and_decrypt(msg, iterations)


ALPHABET = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456789?!*_:;!%&/()=?"


def encrypt_and_decrypt(text, iterations):
    """
    Encrypts and decrypts the given string for a given number of times.
    :param text: text to be encrypted and decrypted.
    :param iterations: number of times to encrypt and decrypt.
    """
    key = os.urandom(16)
    msg = text.encode("utf-8")
    for _ in range(iterations):
        aes = pyaes.AESModeOfOperationCTR(key)
        cipher = aes.encrypt(msg)

        aes = pyaes.AESModeOfOperationCTR(key)
        msg = aes.decrypt(cipher)
    msg = msg.decode("utf-8")
    print(f"decrypted == original text? {msg == text}")


def generate_random_string(length):
    return ''.join(random.choice(ALPHABET) for _ in range(length))
