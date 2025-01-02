# Day 20: [Race Condition](https://adventofcode.com/2024/day/20)

## Part 1

The map here is the same as [Day 16](../day16/README.md), so I could have copied most of the Tile/Direction/Grid code. But for a change, I decided to without the Tile and Direction enum this time.

When I initially implemented part 1, I had misread the description and I solved it with a Dijkstra for getting the path, and trying to place a hole in each wall. This worked for part 1 but was completely the wrong thing for part 2.

## Part 2

After a break, I analyzed again and realized I had misunderstood the problem.

I made a few important observations:

- There is only one path in the maze from start to end. There is no need to bother with Dijkstra.
- When cheating, it means we can jump to any track space around within a circle of X picoseconds.
- We can cheat a maximum of one time.

So I took following approach:

First I find the bath bath, simply walking through the maze.

Then I go through this path, and on each position, I find all possible cheat destinations. Cheat destinations are all track positions in a circle of size 20.

For each cheat destination, we know the cost to the end, it's the remaining path length. So for each cheat, it's easy to calculate the full path cost and find how many cheats are saving at least 100 picoseconds.

Solution is a bit slow for now, around 5 seconds.