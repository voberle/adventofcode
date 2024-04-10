# Day 10: [Adapter Array](https://adventofcode.com/2020/day/10)

## Part 1

This is a problem of creating a graph and visiting all the nodes.

The graph is directed and doesn't have cycles. So I used a simple recursive method, stopping once I have a path that has the required length. If the list of adapters is sorted (important), then it works quickly.

## Part 2

For second part, like the description hints, brute-force is not an option.

Instead I recognized there were some patterns in the graph. Precisely, there were 3 types of patterns, each, corresponding to a number of options. Finding all the patterns and multiplying their number of options gives the answer.

It was useful to have 2 examples to confirm that theory.