# Day 24: [It Hangs in the Balance](https://adventofcode.com/2015/day/24)

## Part 1

A difficult one.
With a small input analysis, I found what was the smallest group 1 size possible, and then checked if any of these group options would have a valid weight sum.
For those that did, I ordered them by increased product, and for each I checked if the rest could be divided into two equal partitions and picked the first.
For checking the equal partitions, I used a dynamic programming implementation of the partition problem.

## Part 2

Part 2 required using a 3-partition algorithm, which I got by first switching to an optimized version of the 2-partition one, and then converting the 2-partition into a 3-partition one.