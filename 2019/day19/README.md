# Day 19: [Tractor Beam](https://adventofcode.com/2019/day/19)

## Part 1

Intcode again. Nothing complicated in part 1, besides nothing that the computer needs to be restarted for each attempt.

## Part 2

My initial solution was to look on the printout for a position within the beam and use that to find another position in the beam much further, where the ship fits for sure.
I later removed this hard-coding of the initial position.

Then progressively get the ship as high as possible.

I got stuck for a while as I only tried to move the ship up or left, but at the end you could squeeze it more up by moving it diagonally.