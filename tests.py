#!/usr/bin/env python3

from collections import defaultdict

with open('graph.txt') as f:
  lines = f.readlines()

graph_egdes = [line.strip().split() for line in lines]

graph = defaultdict(lambda: [])

for edge in graph_egdes:
  graph[edge[0]].append(edge[1])
  graph[edge[1]].append(edge[0])

with open('tree.txt') as f:
  lines = f.readlines()

graph_tree = [line.strip().split() for line in lines if line.strip()]

# Assert that every vertex is used at most once.
assert len(set([pair[1] for pair in graph_tree])) == len(graph_tree)

# Assert that no vertex points to itself.
for edge in graph_tree:
  assert edge[0] is not edge[1]

# Assert that all used edges are in the graph.
for edge in graph_tree:
  assert edge[0] in graph[edge[1]]

print("The tree is valid.")
