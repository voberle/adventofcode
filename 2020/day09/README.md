# Day 9: [Encoding Error](https://adventofcode.com/2020/day/9)

## Part 1

Part 1 was easy to brute force with iterators.

## Part 2

That part brute forces itself well too.

One interesting fact is that `tuple_combinations()` is quite a bit faster than `combinations()`, which makes sense since it avoids a lot of vec allocations.