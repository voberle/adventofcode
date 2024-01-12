# Day 14: [One-Time Pad](https://adventofcode.com/2016/day/14)

## Part 1

The nice part of this implementation is the use of a cache in order to compute each has only once, and also using lifetimes to return a reference to the cache value instead of a copy.

## Part 2

