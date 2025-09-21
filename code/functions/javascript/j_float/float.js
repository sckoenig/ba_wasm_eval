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
    const res = floatOps(n);
    console.log(res);
}

function floatOps(n) {
    let sum = 0.0;
    for (let i = 0; i < n; i++) {
        sum += Math.sin(i);
        sum += Math.cos(i);
        sum += Math.sqrt(i);
    }
    return sum;
}
