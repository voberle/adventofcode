# Day 24: [Never Tell Me The Odds](https://adventofcode.com/2023/day/24)

## Part 1

Part 1 was mainly about remembering the equations of a line in 2D, and calculating if two of these lines cross. Considering only intersections in future added some work, but the task fortunately provided a lot of test cases to validate it. Then looping on all possible line combinations gave the answer.

## Part 2

Part 2 was too difficult for me.

On AoC Reddit, I learned about the trick of changing the reference frame so that the rock doesn't move. Also the [solution from encse](https://aoc.csokavar.hu/?day=24) helped understand how to reuse part 1 to implement it, by projecting the coordinates on XY and then on XZ.