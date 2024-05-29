# Day 3: [Rucksack Reorganization](https://adventofcode.com/2022/day/3)

## Part 1

It was easy with HashSet. Maybe worth optimizing.

## Part 2

The part 2 code doesn't look nice, because I had split the rucksacks into two when building them.

Execution time: 390 µs

## Update 1

I completely refactored the solution to remove the HashSet and use a custom solution to find the intersections. This speeds things up, but not that significantly as I had hoped.

Execution time: 260 µs

## Update 2

Replacing the vector with arrays helped as well.

Execution time: 190 µs