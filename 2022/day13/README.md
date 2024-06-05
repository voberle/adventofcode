# Day 13: [Distress Signal](https://adventofcode.com/2022/day/13)

## Part 1

This was a case where the parsing was rather tricky. In [2021 Day 18](../../2021/day18/README.md) I could use the assumption that all numbers are single digits, but here we have number as high as 10! So to keep things simple, I opted for first parsing each paquet into an intermediary list of tokens (and dealing with the parsing of the numbers at that level) and the building the list of signals in a recursive way like in [2021 Day 18](../../2021/day18/README.md) or in [2022 Day 07](../day07/README.md).

Once that was done, solving part 1 was a matter of implementing carefully the instructions. Rust strong typing helps to avoid silly mistakes here, and I nicely got the right answer on first attempt, bug free!

The code is clean and readable.

## Part 2

For part 2, we need to sort the signals, so I convered the compare method to the `Ord` trait, to be able to use the sort method. This actually made the comparaison code even shorter. The whole thing is quite elegant now.