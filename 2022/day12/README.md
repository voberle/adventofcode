# Day 12: [Hill Climbing Algorithm](https://adventofcode.com/2022/day/12)

## Part 1

That was a simple case of applying Dijkstra shortest path algorithm.

## Part 2

Fortunately part 2 brute-forces itself very well.

It runs in 65 ms, which is surprisingly fast considering that I'm trying all possible starting points in part 2.

## Update

From the solutions mega-thread, I saw that a more optimal version for part 2 was to start from the end, then we can just find the shortest path to elevation 0.

This brings it down to 2 ms.