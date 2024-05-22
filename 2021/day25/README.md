# Day 25: [Sea Cucumber](https://adventofcode.com/2021/day/25)

## Part 1

Surprisingly, I struggled a bit with this one, with getting the moving methods correct using my grid with a single vector. At the end, I used a simpler maybe more naive approach, but that works.

It would be nice to get a version that doesn't clone the grid on each iteration though. Currently it runs in 33 ms.

## Update

I removed the clone() and used hash instead (with FxHasher) to check if the grid has changed, and it's actually SLOWER than with the clone, taking 44 ms. So cloning a big vector is faster than hashing it.