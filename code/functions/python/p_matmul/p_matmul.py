import sys


def main():
    """
    Implements a matrix multiplication benchmark.
    Two quadratic matrices are first generated pseudorandomly and then multiplied
    and the result printed to stdout.

    Inspired by:
    Vojdan Kjorveziroski & Sonja Filiposka 2023
    https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/matmul/main.go
    """

    args = sys.argv[1:]
    if len(args) != 1:
        print("Pass a number as argument.", file=sys.stderr)
        return
    try:
        dim = int(args[0])
    except ValueError:
        print("Pass a number as argument.", file=sys.stderr)
        return

    a = generate_matrix(dim, SEED1)
    b = generate_matrix(dim, SEED2)
    c = mat_mul(a, b)
    print(f"Res: {c[:1]} ...")


def mat_mul(a, b):
    """"
    Multiplies two matrices, creating a new matrix.
    :param a: first matrix a
    :param b: second matrix b
    :return: new matrix c
    """
    n, m, p = len(a), len(b), len(b[0])
    c = [[0.0] * p for _ in range(n)]

    for i in range(n):
        for j in range(p):
            for k in range(m):
                c[i][j] += a[i][k] * b[k][j]

    return c


SEED1 = 32
SEED2 = 64
M = 1 << 48
C = 11
A = 25214903917


class LCG:
    """
    Linear congruential generator for pseudorandom number generation.
    https://en.wikipedia.org/wiki/Linear_congruential_generator
    """

    def __init__(self, seed):
        self.seed = seed

    def next_f64(self):
        """
        Returns the next pseudorandom float using Xn+1 = (A * Xn + C) % M
        """
        self.seed = (A * self.seed + C) % M
        return self.seed / M


def generate_matrix(dim, seed):
    """
    Generates a quadratic matrix filled with pseudorandom floats.
    :param dim: dimension of matrix
    :param seed: random seed
    :return: dim x dim matrix
    """
    rng = LCG(seed)
    return [[rng.next_f64() for _ in range(dim)] for _ in range(dim)]
