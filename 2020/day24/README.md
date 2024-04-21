# Day 24: [Lobby Layout](https://adventofcode.com/2020/day/24)

## Part 1

This is a hexa grid, like in [2017 Day 11](../../2017/day11/README.md). I reused the same Cube coordinates mechanism.

## Part 2

Calculating the new floor each time is fairly easy.

Curiously, I found the answer while having two bugs, which neutralized each other:

- In the first condition, I counted black tiles as `black_count >= 2` instead of `black_count > 2`.
- In the second condition, I forgot to filter out black tiles.
