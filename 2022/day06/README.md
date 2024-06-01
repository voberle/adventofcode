# Day 6: [Tuning Trouble](https://adventofcode.com/2022/day/6)

## Part 1

Simple enough with the windows() iterator.

## Part 2

I had to make a smarter all_different method, but nothing complicated.

Part 2 runs in about 25 µs.

## Update

ChatGPT suggests that using a HashSet for the all_different method would be faster, O(n) instead of O(n^2), but in practice it's 10 times slower.

However doing the same idea with an array is at least as fast or a tad better: 22 µs.

Replacing the boolean array with an integer is better: 17 µs.