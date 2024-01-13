# Day 11: [Radioisotope Thermoelectric Generators](https://adventofcode.com/2016/day/11)

## Part 1

Suddenly the level of difficulty jumped a lot.

First modeling the building and the elevator took some time to get right.

Then the recursive approach was tricky to get working. The first attempt that worked I had to use an arbitrary limit in how far the recursion would go, and that allowed it to find the right answer.

## Part 2

The simpler version of the recursive approach that worked for part 1 didn't work for the bigger input of part 2.

So I had the idea to look at which best method the computer found for part 1, and if there were any optimizations I could deduct from it. And indeed they were:

- Down moves was always limited to 1 element only, we never took down 2 elements at once.
- Up moves was the opposite, we took always 2 elements (note that this was not true for the test input, where first move was Up with 1 only).

Adding these two limitations to the possible moves and it solves part 2 as well!