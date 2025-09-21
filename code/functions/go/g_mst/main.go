// Implements a graph benchmark, first
// creating a random weighted graph and then calculating
// the minimum spanning tree of that graph.
//
// Inspired by:
// SeBS: Serverless Benchmark Suite
// https://github.com/spcl/serverless-benchmarks/blob/master/benchmarks/500.scientific/502.graph-mst/python/function.py

package main

import (
	"bench"
	"fmt"
	"github.com/dominikbraun/graph"
	"math/rand"
	"os"
	"strconv"
)

func main() {
	bench.Benchmark(func() {
		if len(os.Args) != 2 {
			_, _ = fmt.Fprintf(os.Stderr, "Usage: g_mst NODE_COUNT")
			return
		}
		nodeCount, err := strconv.Atoi(os.Args[1])
		if err != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			return
		}
		g := generateRandomGraph(nodeCount)

		mst, _ := graph.MinimumSpanningTree(g)
		edges, _ := mst.Edges()

		fmt.Printf("%v ... \n", edges[:min(32, len(edges))])
	})
}

// generateRandomGraph generates a graph with n nodes and random positive weights.
func generateRandomGraph(n int) graph.Graph[int, int] {
	g := graph.New(graph.IntHash)
	for i := 0; i < n; i++ {
		_ = g.AddVertex(i) // err if node exists
	}

	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			_ = g.AddEdge(i, j, graph.EdgeWeight(rand.Int())) // err if node does not exist or edge already exists
		}
	}
	return g
}
