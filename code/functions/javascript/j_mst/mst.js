import jsgraphs from "js-graph-algorithms";

/**
 * Implements a graph benchmark, first
 * creating a random weighted graph and then calculating
 * the minimum spanning tree of that graph using
 * the Kruskal algorithm.
 *
 * Inspired by:
 * SeBS: Serverless Benchmark Suite
 * https://github.com/spcl/serverless-benchmarks/blob/master/benchmarks/500.scientific/502.graph-mst/python/function.py
 */
export function main(n) {
    const graph = generateRandomGraph(n);
    const kruskal = new jsgraphs.KruskalMST(graph);
    const mst = kruskal.mst
    console.log(mst.slice(0, 32));
}

/**
 * Generates a graph with random positive weights in [0,1).
 */
function generateRandomGraph(node_count) {
    const g = new jsgraphs.WeightedGraph(node_count);
    for (let i = 0; i < node_count; i++) {
        for (let j = i + 1; j < node_count; j++) {
            g.addEdge(new jsgraphs.Edge(i, j, Math.random()));
        }
    }
    return g;
}