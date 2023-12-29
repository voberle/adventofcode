# Day 21: [Step Counter](https://adventofcode.com/2023/day/21)

## Part 1

For part 1, for each step, it was just necessary to go through the whole grid and toggle the neighbours.

Once the grid is full, it oscillates between two values, so I implemented detecting this and using it as an optimization, even if it didn't kick in for the real input unfortunately.

## Part 2

For part 2, we had to notice that the real input had a special pattern, as a diamond, with the middle vertical and horizontal lines all empty.

I first tried a solution where I would virtually expand the garden as needed and count.. This didn't work out at all, as it was way too complicated to debug, and probably didn't scale.

So a look on Reddit AoC made me notice the other important information: The number of steps was special:

    26501365 = 65 + (202300 * 131)

where 131 is the width of the grid, and 65 the number of steps it takes to go from the center to the edge.

The [diagram from villuna](https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21) helped me understand the geometric approach, which sounded easy enough. After a few trials, I got the right number.