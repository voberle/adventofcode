# Day 14: [One-Time Pad](https://adventofcode.com/2016/day/14)

## Part 1

The nice part of this implementation is the use of a cache in order to compute each has only once, and also using lifetimes to return a reference to the cache value instead of a copy.

## Part 2

In part 2, there are all the intermediate 2016 hashes to calculate, where there is probably nothing extra to optimize, so it's bit slow.

Main challenge there was properly converting the data for the MD5 method. But once I had it, I simplified the code and it turned out surprisingly simple.