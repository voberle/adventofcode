# Day 23: [Safe Cracking](https://adventofcode.com/2016/day/23)

## Part 1

It's an extension to [Day 12](/../day12/README.md), all that code could be reused, and fortunately the initial implementation worked mostly.

I just had to make the instructions mutable, and handle a few variations of jump.

I later cleaned up the code by using a `IntChar` enum, to store an interger or a char. This removes the duplicate Instruction and makes the code cleaner.

## Part 2

The description hints that an optimization is needed, but my version runs without it in a few seconds.