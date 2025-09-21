import math
import sys


def main():
    """
    Implements a float operation benchmark,
    calculating the sin, cos and square root of n numbers.

    Adapted from:
    FunctionBench
    https://github.com/ddps-lab/serverless-faas-workbench/blob/master/aws/cpu-memory/float_operation/lambda_function.py
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

    res = float_ops(n)
    print(res)


def float_ops(n):
    res = 0.0
    for i in range(0, n):
        res += math.sin(i)
        res += math.cos(i)
        res += math.sqrt(i)
    return res
