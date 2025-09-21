import math
import sys


def main():
    """
    Implements a prime benchmark. Calculates the first n prime numbers
    and printing them to stdout.

    Inspired by:
    Vojdan Kjorveziroski & Sonja Filiposka 2023
    https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/matmul/main.go
    """
    args = sys.argv[1:]
    if len(args) < 1:
        print("Pass a number as argument.", file=sys.stderr)
        return
    try:
        n = int(args[0])
    except ValueError:
        print("Pass a number as argument.", file=sys.stderr)
        return

    res = prime(n)
    print(f"{res[:200]} ...")


def prime(n):
    """
    Calculates the first n prime numbers using the Sieve of Eratosthenes.
    :param n: max
    :return: list of prime numbers <= n
    """
    is_prime = [True] * (n + 1)

    if n < 2: return []
    is_prime[0] = is_prime[1] = False

    sqrt = math.isqrt(n)
    for i in range(2, sqrt + 1):
        if is_prime[i]:
            for j in range(i * i, n + 1, i):
                is_prime[j] = False

    return [i for i in range(2, n + 1) if is_prime[i]]
