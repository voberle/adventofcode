# Day 21: [Monkey Math](https://adventofcode.com/2022/day/21)

## Part 1

Nothing too complicated there, we are going over the operations, calculating and removing the ones we can, until none is left.

## Part 2

The approach there is to solve all equations progressively.

First I simplified as much as possible. This made appear that one of the root element had a value, so we got the value of the other one. This allowed to simplified further.

In part 1 I had support for calculating:

    monkey: val op val

In part 2 I added support for solving when we know the first part, like:

    val: monkey op val

Both parts run in 3.5 ms.