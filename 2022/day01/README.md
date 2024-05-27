# Day 1: [Calorie Counting](https://adventofcode.com/2022/day/1)

## Part 1

What is the most efficient and most elegant to parse such an input of blocks of numbers separated by empty lines?

## Part 2

Simple stuff.

## Performance

I optimized the solution to build immediately a vector of sums, and sort it. That runs in *38 µs* instead of *65 µs*.

I also tried a version with `BinaryHeap` (which avoids having to sort the vector), but it's slower, *68 µs*.

A shorter parsing approach like the following is also slower, *55 µs*:

    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|food| food.parse::<u32>().unwrap_or(0)).sum()
        })
        .sorted().rev().collect()
