# Day 22: [Mode Maze](https://adventofcode.com/2018/day/22)

## Part 1

Part 1 was mainly about building the map by following carefully the instructions.

## Part 2

Before digging into the path finding, I refactored the part 1 code to make the cave auto-growing, and separated it into a module to make it clear which functions the path finding needed.

I also avoided exposing the auto-growing feature by mutting the hash map in a `RefCell`, so that the cave doesn't need to be mutable.

Then it was a matter of running Dijkstra shortest path algorithm on it, by looking at each node as a pair of position + tool (meaning each position gives us 3 cases to explore).

3 other details were important:

- As the map can grow indefinitely, I had to limit how far we search.
- Correctly implementing the tool selection logic, as the new tool must also be valid for the _current_ region (test input passed without this).
- Make sure we have the torch when reaching the target.