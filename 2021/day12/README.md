# Day 12: [Passage Pathing](https://adventofcode.com/2021/day/12)

## Part 1

This is a problem of finding all possible paths connecting two nodes in an undirected graph.

I implemented a recursive Depth First Traversal (DFS) of the graph, which worked beautifully

I built the graph as vector of caves, with each caves having a list of the indexes of the connected caves. This is working well with Rust borrow checker and should be very efficient.

My solution runs in 2 ms.

## Part 2

Surprisingly, it was almost harder to implement part 2 than 1, as I struggled to find the right abstractions for the visit tracking.

Current solution uses a trait for visit tracking, and two different implementations. They could easily be merged into one.

## Update

After having a look at other solutions, I realized there was no need to save all paths found, but I just needed to counted the number of paths.

This divided the runtime by 3, to 10ms for both parts.

I also realized that it would be possible to get read of the visited structure fully, and just check the path under construction to see if a cave was already visited. This would result in shorter code, but might not be more readable. It might also be slower, as I would replace some direct memory access with a search in a vector.

However, the path itself doesn't need to be saved, since we only need to detect when we reach the end. So removed this as well.