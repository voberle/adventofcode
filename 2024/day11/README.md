# Day 11: [Plutonian Pebbles](https://adventofcode.com/2024/day/11)

## Part 1

Implemented part 1 in a brute-force way, knowing perfectly that in AoC spirit, it's unlikely to work for part 2.

I'm not sure that the way I split numbers is the best one.

Switching from a flat_map to a simple for loop in the blink method divided the run time by 2 (16 to 8 ms), since we removed the allocation of many small vectors.

## Part 2

