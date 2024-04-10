# Day 10: [Adapter Array](https://adventofcode.com/2020/day/10)

## Part 1

This is a problem of creating a graph and visiting all the nodes.

The graph is directed and doesn't have cycles. So I used a simple recursive method, stopping once I have a path that has the required length. If the list of adapters is sorted (important), then it works quickly.

## Part 2

