# Day 6: [Chronal Coordinates](https://adventofcode.com/2018/day/6)

## Part 1

The easiest seems to be to mark each point in the grid with the coords that they are closest, and do this, we check each coords by going from smallest distance to bigger.

I attempted to use a vector for the grid, but it turned out to complicated, and a FxHashMap was clearly fast enough in the end.

## Part 2

