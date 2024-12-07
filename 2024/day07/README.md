# Day 7: [Bridge Repair](https://adventofcode.com/2024/day/7)

## Part 1

Brute-forcing it by trying all possible combinations of operators works well here. Part 1 runs in 10 ms.

## Part 2

Fortunately part 2 can still be brute-forced, the whole things running in 360 ms.

Optimizing concatenation brings it down to 340 ms. Interrupting the iteration when the total is bigger than the expected result brings it down to 313 ms.