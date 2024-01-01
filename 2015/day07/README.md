# Day 7: [Some Assembly Required](https://adventofcode.com/2015/day/7)

## Part 1

Main challenge in this task was modelizing it in a way that made Rust happy. A first attempt with a mutable Circuit created too much trouble with borrow checker. Keeping Circuit immutable kept things simpler.

Then the other work was to properly support all the features of the gates, including that gates could have signals or wires as inputs.

## Part 2

Fairly simple with the part 1 implementation.