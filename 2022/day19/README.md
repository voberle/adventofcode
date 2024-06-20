# Day 19: [Not Enough Minerals](https://adventofcode.com/2022/day/19)

## Part 1

Like [Day 16](../day16/README.md), which I also struggled with, this is about maximizing a value in a specific amount of time.

Initially I had started with a recursive approach, but it was hard to limit it, so I decided to use the same approach as on day 16, going minute by minute and keep a set of all possible states.

Implementing the logic isn't very hard then, and with some simple optimizations (not buying robots in last minutes), it runs fast enough to get the answer in 14 seconds.

## Part 2

For second part, it was fortunately enough to just prune more aggressively the list of states. Dropping the states if the number of found geodes is less than the max found so far minus 3 was enough to get part 2 to complete in 20 seconds.

More aggressive pruning also works: Changing the value from 3 to 0 allows both parts to run in under 9 seconds.