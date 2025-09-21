/**
 * Implements a float operation benchmark,
 * calculating the sin, cos and square root of n numbers.
 *
 *
 * Adapted from:
 * FunctionBench
 * https://github.com/ddps-lab/serverless-faas-workbench/blob/master/aws/cpu-memory/float_operation/lambda_function.py
 */
export function main(n) {
    const res = fibonacci(n);
    console.log(res);
}

function fibonacci(n) {
    if (n <= 1) {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
