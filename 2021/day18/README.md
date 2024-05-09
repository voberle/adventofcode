# Day 18: [Snailfish](https://adventofcode.com/2021/day/18)

## Part 1

This task was mainly about the coding challenge.

The parsing of the data and figuring out the data structure to use wasn't trivial. The data structure I ended up using worked fairly well. I did the parsing moving by index, which may not be the most elegant, but is simple enough and works.

The real difficulty was implementing the explosion in Rust. The borrow checker made it very complicated to find what to explode and do it at the same time. So I split the problem in two, first finding what to explode, and then rebuilding the number with the exploded data.

Splitting, addition and magnitude support was easier.

All this data copying doesn't make it the fastest probably, runs in 14 ms currently.

## Part 2

Fortunately part 2 was trivial.