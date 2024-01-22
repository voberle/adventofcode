# Day 17: [Spinlock](https://adventofcode.com/2017/day/17)

## Part 1

Part 1 was simple, with `rem_euclid`.

## Part 2

Part 2 had me wondering for a bit. The trick is that we want the value after 0, and *0 never moves*, so it means we want the value at index 1. So we don't need to actually have a vector and do inserts, we can just track which value would be set at index 1.