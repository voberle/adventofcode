# Day 15: [Oxygen System](https://adventofcode.com/2019/day/15)

## Part 1

The main work was building the maze. Since we cannot specify a position to the computer, we cannot jump anywhere, but always have to move. So building the maze cannot be done with recursion, but in an iterative way with a moving back mechanism.

Once the maze was built, finding the shortest path was simple with Dijkstra.

## Part 2

