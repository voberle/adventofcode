# Day 6: [Universal Orbit Map](https://adventofcode.com/2019/day/6)

## Part 1

Orbits make a graph, so it's just a matter of counting the number of connections in the graph.

My implementation is not the most efficient, as I don't cache already made counts. But it takes only 7 ms.

## Part 2

Part 2 was simpler than it felt initially. We just need to find the path to the center for each, then find the intersection and count the steps to it.
