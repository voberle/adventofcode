# Day 7: [The Sum of Its Parts](https://adventofcode.com/2018/day/7)

## Part 1

Solved this one by having only one short array of length 26, which tracks which steps have been executed (and their position) and which ones are ready to be executed next.

I also optimized it later with a better data structure for the dependencies list.

## Part 2

I split the part 1 solution in smaller functions, which helped build part 2.