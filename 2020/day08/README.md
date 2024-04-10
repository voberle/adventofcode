# Day 8: [Handheld Halting](https://adventofcode.com/2020/day/8)

## Part 1

This was another little VM to implement, fairly simple.

## Part 2

On reading the description, I thought this might become a complex optimization problem, but it wasn't at all and was trivial to brute force.

At first I ran it with a maximum number of instructions to execute before giving up, which gave the answer.

Later I realized you could just reuse part 1 to detect when to give up, since when run an instruction again, it means we entered a loop.