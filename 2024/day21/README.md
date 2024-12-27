# Day 21: [Keypad Conundrum](https://adventofcode.com/2024/day/21)

## Part 1

Here is one of those problems that require some real thinking to figure out the design.

I wrote out a model for the keypads, which mainly give the paths to go from one key to another.

Then I build all possible directions phase by phase.

It's not very fast however: 7 seconds.

## Part 2

I was stuck for a while there, as the approach I took in part 1 generated too many different paths on each step, so that it simply wasn't scaling.

The insight came when I noticed following: For going to 2 for 9, we prefer ^^>A over ^>^A, since the first one just allows to press twice the A on the same key.

This allowed me to first limit the number of options we have when going from direction to direction.