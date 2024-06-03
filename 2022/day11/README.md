# Day 11: [Monkey in the Middle](https://adventofcode.com/2022/day/11)

## Part 1

Most code went into parsing the input. Implementing the money business was fairly simple.

## Part 2

I was actually stuck on a while on this one.

Brute-forcing it didn't work at all, even using some of the big int crates. When not crashing, it was getting very slow after less than 1000 iterations.

I realized that using modulo was likely the way, but my understanding of modular arithmetic was incomplete so I didn't figure out how. When I finally read more about [Modular Arithmetic](https://brilliant.org/wiki/modular-arithmetic/) I understand how I could use to keep the worry levels low. Finding by which number to the modulo came quickly through intuition.
