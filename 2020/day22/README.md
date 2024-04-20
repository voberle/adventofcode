# Day 22: [Crab Combat](https://adventofcode.com/2020/day/22)

## Part 1

Nothing complicated in part 1.

## Part 2

Part 2 was mostly about understanding the instructions and manipulating the `VecDeque` correctly.

## Update

I improved the infinite loop detection by not storing the full deck in the set, but only its hash. This sped things up quite a bit. Initially it ran in 38 ms. After the change, it came down to 16 ms.
