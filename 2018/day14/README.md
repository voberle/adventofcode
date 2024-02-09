# Day 14: [Chocolate Charts](https://adventofcode.com/2018/day/14)

## Part 1

Part 1 was easy. Runs in 11ms.

## Part 2

Part 2 brute forces nicely as well, with same algorith and slice comparaison. Runs in 460 ms.

After I refactored the code to have both parts share the same function that implements the main algorithm, and using a `FnMut` to decide when to stop looping. Looks cool!