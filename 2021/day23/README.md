# Day 23: [Amphipod](https://adventofcode.com/2021/day/23)

## Part 1

The approach I took was to encode all possible positions as a short vector. Then for each vector, I have long and not so smart code that finds the next possible vectors that we can get from there. Finally, I apply Dijkstra to find the shortest path to the final state.

While the next positions code is long and ugly, the final solution works nicely and is fast (600 ms).

## Part 2

