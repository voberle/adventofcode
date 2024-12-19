# Day 19: [Linen Layout](https://adventofcode.com/2024/day/19)

## Part 1

It didn't feel so easy, even if my solution looks fairly easy finally.

The idea is that I go through the pattern in a recursive way, checking if I can find a towel set. If I do, I check the rest of the pattern by calling the same function recursively.

It worked once I added memoization. Without it, the impossible patterns never stop.

It's nicely fast, 6 ms.

## Part 2

Part 2 was very easy: Just by having part 1 code return a count instead of a boolean, it worked.