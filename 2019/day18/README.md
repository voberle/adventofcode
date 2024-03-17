# Day 18: [Many-Worlds Interpretation](https://adventofcode.com/2019/day/18)

## Part 1

That is a difficult one, and my solution so far is probably too complicated and clearly too slow.

I use Dijkstra to find the shortest path, and recursion to explore all options.

With some caching, that works fairly quickly for the test inputs, but for the real one it doesn't stop. But printing out the shortest path found so far, and waiting a bit under a minute gives the right answer.

I haven't found how to optimize it better for now.

## Part 2

