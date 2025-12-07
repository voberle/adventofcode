# Day 7: [Laboratories](https://adventofcode.com/2025/day/7)

## Part 1

Defining the beams line by line worked well.

## Part 2

Part 2 was the hardest one so far this year.

I felt a recursive exploration of the manifold would be the easiest, but doing that on the grid felt complicated and inefficient. So I converted the grid to a graph, which was the most work. Then exploring the graph recursively is fairly easy. Adding memoization via a cache made it work with the real input.