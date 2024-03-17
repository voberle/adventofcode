# Day 18: [Many-Worlds Interpretation](https://adventofcode.com/2019/day/18)

## Part 1

That is a difficult one, and my solution so far is probably too complicated and clearly too slow.

I use Dijkstra to find the shortest path, and recursion to explore all options.

With some caching, that works fairly quickly for the test inputs, but for the real one it doesn't stop. But printing out the shortest path found so far, and waiting a bit under a minute gives the right answer.

I haven't found how to optimize it better for now.

## Part 2

Modifying the code to handle 4 entrance positions works nicely.

It's actually even faster than for part 1.

But the recursion is still not stopping for the real input (does for tests). I'm stuck as to how to fix that.
