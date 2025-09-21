import sys
from random import random

import networkx as nx


def main():
    """
    Implements a graph benchmark, first
    creating a random weighted graph and then calculating
    the minimum spanning tree of that graph.

    Inspired by:
    SeBS: Serverless Benchmark Suite
    https://github.com/spcl/serverless-benchmarks/blob/master/benchmarks/500.scientific/502.graph-mst/python/function.py
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

    graph = generate_random_graph(n)
    res = nx.minimum_spanning_tree(graph)
    print(f"{list(res.edges)[:32]} ...")


def generate_random_graph(n):
    g = nx.Graph()
    for i in range(n):
        for j in range(i + 1, n):
            g.add_edge(i, j, weight=random())
    return g
