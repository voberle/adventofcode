# Day 20: [Donut Maze](https://adventofcode.com/2019/day/20)

## Part 1

Not specially difficult a priori, but it took some time due to the parsing of the map. I parsed it in 3 steps:

1. Extract the positions of each portal.
2. Parse the maze ignoring the portals.
3. Extend the maze with the portals.

Once that was done, a classic Dijkstra did the job.

## Part 2

Part 2 needed some refactoring in the map building to separate the different types of portals.

Then it was just a matter of adding a dimension to Dijkstra, something that I have done before.
