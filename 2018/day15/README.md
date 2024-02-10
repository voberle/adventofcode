# Day 15: [Beverage Bandits](https://adventofcode.com/2018/day/15)

## Part 1

This one required a lot of code compared to usual.

Rust does help in such cases I think: By having more constraints, it requires to write more robust code, which ends up having less side effects.

The few bugs I had were quick to find:

- I had missed one place to check for dead units, since within a round, I didn't delete the units, but just marked them dead.
- When doing the sum of hit points, I included a dead unit with -1 points.
- I had the end of combat check misplaced, resulting in a number of combat rounds off by 1.
- It was complicated to get the shortest path algorithm to select the path in reading order when they were multiple ones. At first I had modified my Dijkstra implementation and I thought it worked since the test passed, but it didn't. It's actually never a good idea to hack such algorithms, it's better to leave them untouched and change how you use them. I did that, calculating the shortest path from the adjacent positions of the starting location.

## Part 2

