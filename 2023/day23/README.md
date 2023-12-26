# Day 23: [A Long Walk](https://adventofcode.com/2023/day/23)

Initially I got part 1 solved by implementing Dijkstra's algorithm, but it worked only because it was an acyclic graph. As a general method, a *shortest* path algorithm to find the longest path doesn't work.

I then refactored the whole thing to transform the maze into a graph that was easier to process. I validated that the graph was right by printing it out in Graphviz format and visualizing it.

A simple iterative DFS (Depth First Search) didn't work however, it had to be brute forced. However this turned out to be fast, around 2 seconds execution time.