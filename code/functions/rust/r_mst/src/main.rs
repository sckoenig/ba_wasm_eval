//! Implements a graph benchmark, first
//! creating a random weighted graph and then calculating
//! the minimum spanning tree of that graph.
//!
//! Inspired by:
//! SeBS: Serverless Benchmark Suite
//! https://github.com/spcl/serverless-benchmarks/blob/master/benchmarks/500.scientific/502.graph-mst/python/function.py

use bench::*;
use petgraph::algo::min_spanning_tree;
use petgraph::prelude::UnGraph;
use petgraph::Graph;
use rand::Rng;
use std::env;

fn main() {
    benchmark(|| {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 1 {
            eprintln!("Usage: r_mst NODE_COUNT");
            return;
        }
        match args[0].parse() {
            Err(e) => eprintln!("Error: {}", e),
            Ok(n) => {
                let graph = generate_random_graph(n);
                let mst = min_spanning_tree(&graph);
                mst.skip(n)
                    .take(32)
                    .for_each(|edge| println!("{:?})", edge));
                println!("...");
            }
        }
    });
}

/// Generates a graph with random positive weights in [0,1).
fn generate_random_graph(node_count: usize) -> Graph<(), f64, petgraph::Undirected> {
    let mut graph = UnGraph::<(), f64>::new_undirected();
    let nodes: Vec<_> = (0..node_count).map(|_| graph.add_node(())).collect();
    let mut rng = rand::rng();

    for i in 0..node_count {
        for j in (i + 1)..node_count {
            let weight = rng.random::<f64>();
            graph.add_edge(nodes[i], nodes[j], weight);
        }
    }
    graph
}
