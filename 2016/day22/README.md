# Day 22: [Grid Computing](https://adventofcode.com/2016/day/22)

## Part 1

Nothing special here, a simple iterator did the job.

NB: I introduced an `int` method to help with parsing.

## Part 2

The first instinct was to try to draw the graph of viable nodes with Graphviz. This wasn't very interesting, but it made me notice that like in the test data, exactly one node was empty.

The second observation was that a lot of nodes, *but not all* were viable connection to the empty node. So I wondered where those non-viable nodes were and implemented a proper visualization of the graph. It turned out that they form a horizontal line between the empty node and the first line.

From there I figured out that the path could somehow be manually calculated, by following the same approach as described in the task.