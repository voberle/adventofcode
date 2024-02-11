# Day 16: [Chronal Classification](https://adventofcode.com/2018/day/16)

## Part 1

Part 1 was nice to implement with a vector of functions for the instructions.

## Part 2

Building the logic to find the opcodes matching to the instructions was quite fun. I first build a list of possible instructions for each opcode (as we found how to do in part 1). Then I reduce this list progressively by looking for the sure matches (candidate list is only 1) and removing these sure matches from the other lists.