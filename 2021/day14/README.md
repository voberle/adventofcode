# Day 14: [Extended Polymerization](https://adventofcode.com/2021/day/14)

## Part 1

This is one of those problems where a sequence grows quickly. It's a classic where you brute-force part 1 and part 2 reserves you huge numbers that require redoing the whole thing.

Multiple itertools goodies usage here: `split_once`, `collect_tuple`, `minmax_by_key`.

## Part 2

I had the intuition that the template should be split in pairs, as this would allow to easily figure out how many pairs of each type a round generates. Then once I realized that the order of the pairs didn't matter, it was a matter of figuring out how to count the occurences, which was actually easy.

Both parts run in 1.5 ms.