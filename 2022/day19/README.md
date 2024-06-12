# Day 19: [Not Enough Minerals](https://adventofcode.com/2022/day/19)

## Part 1

Like [Day 16](../day16/README.md), which I also struggled with, this is about maximizing a value in a specific amount of time.

Initially I had started with a recursive approach, but it was hard to limit it, so I decided to use the same approach as on day 16, going minute by minute and keep a set of all possible states.

Implementing the logic isn't very hard then, and with some simple optimizations (not buying robots in last minutes), it runs fast enough to get the answer in 14 seconds.

## Part 2

