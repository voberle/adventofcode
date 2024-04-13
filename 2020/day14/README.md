# Day 14: [Docking Data](https://adventofcode.com/2020/day/14)

## Part 1

There were two main challenges in this part:

First one was how to apply the bitmask in the simplest way. My solution doesn't use an external crate and looks fairly small.

Second one was how to have a reference to the bitmask when applying the instructions, and avoid doing a copy of it. Here I use the first time [MaybeUninit](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html) and `unsafe`.

## Part 2

