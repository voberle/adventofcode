# Day 10: [Knot Hash](https://adventofcode.com/2017/day/10)

## Part 1

Reversing the sub-list of the circular list was a bit tricky to get the indexes right.

## Part 2

The hardest part was actually reading the very long description, and figuring out how to print the hexadecimal in the correct format.

Note that clippy taught me that my wrapping_index functions can just be replaced with `i.rem_euclid(len)`.