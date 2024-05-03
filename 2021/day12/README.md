# Day 12: [Passage Pathing](https://adventofcode.com/2021/day/12)

## Part 1

This is a problem of finding all possible paths connecting two nodes in an undirected graph.

I implemented a recursive Depth First Traversal (DFS) of the graph, which worked beautifully

I built the graph as vector of caves, with each caves having a list of the indexes of the connected caves. This is working well with Rust borrow checker and should be very efficient.

My solution runs in 2 ms.

## Part 2

Surprisingly, it was almost harder to implement part 2 than 1, as I struggled to find the right abstractions for the visit tracking.