# Day 24: [Planet of Discord](https://adventofcode.com/2019/day/24)

## Part 1

Nothing special in part 1, we have done such things before.

Only point worth noting is that we can use the diversity rating as hash for the scan, and storing this in the set is faster than the full scan.

## Part 2

Part 2 was mostly about writing correctly the function that counts adjacent bugs in 3D.

I use a VecDeque to hold the space, as this allows to add scans easily on both sides.