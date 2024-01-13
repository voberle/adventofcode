# Day 19: [An Elephant Named Joseph](https://adventofcode.com/2016/day/19)

## Part 1

It was actually easy once I realized I didn't need to bother with how many presents each elf had, and just needed a boolean for each elf indicating if they had presents or not. Then each elf marks the next true as false.

Then while working on part 2, I optimized this by not having a vector of boolean, but one indicating the index of the next elf that has presents. This divided the execution time of part 1 by 20!

## Part 2

Part 2 on the other hand is ultra-ugly. I didn't manage to find an efficient version, so I fell back on a brute-force version that removes the elf from the `Vec` on each iteration. Not efficient at all, but got the job done in around 15 minutes.

Later checked AoC Reddit for better ideas, seems for Rust most practical is to split the list in two Vector. I implemented such a version, it's very fast then.