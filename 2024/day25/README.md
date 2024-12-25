# Day 25: [Code Chronicle](https://adventofcode.com/2024/day/25)

## Part 1

This was an easier one to finish the year.

While not necessary, I still parsed the input in a vector of `Grid` first, to make things simpler.

I then converted it into two list of heights, counting the heights with `take_while`:

- I detect if it's a lock or a key by just checking if the first line is full of '#'.
- I split the iterator into two vector with itertools `partition_map`.

Finally to check if a lock fits a key, I chech if the sum of each corresponding heights is lower than 6.