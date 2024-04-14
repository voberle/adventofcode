# Day 15: [Rambunctious Recitation](https://adventofcode.com/2020/day/15)

## Part 1

First part is fairly easy, once you get the indexes right.

## Part 2

I assumed the code in first part with a vector was going to be too slow, so I converted the whole part 1 to track the position of the last occurence of each number. It took a few tries to get it right. It runs in less than 1 second.

I experimented switching to a vector instead of hashmap to track the positions, and it still divided the time by 2.
