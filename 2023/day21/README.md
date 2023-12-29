# Day 21: [Step Counter](https://adventofcode.com/2023/day/21)

## Part 1

For part 1, for each step, it was just necessary to go through the whole grid and toggle the neighbours.

Once the grid is full, it oscillates between two values, so I implemented detecting this and using it as an optimization, even if it didn't kick in for the real input unfortunately.

## Part 2

For part 2, we had to notice that the real input had a special pattern, as a diamond, with the middle vertical and horizontal lines all empty.
