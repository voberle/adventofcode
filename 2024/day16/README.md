# Day 16: [Reindeer Maze](https://adventofcode.com/2024/day/16)

## Part 1

Not very hard with Dijkstra, and using an extended position of "position + direction".

## Part 2

In part 2, we need to find all the shortest paths, with their positions.

This was difficult. I tried extending Dijkstra, which I didn't manage at first. I tried BFS, but that didn't work either. So I went back to Dijkstra, which I ultimately got working.

The principle is to add a predecessors map to the algorithm, which for each node stores ALL the nodes that in a path that is the shortest to this node.

Then once the algorithm is complete, we use this predecessors map to build the list of paths going to it.