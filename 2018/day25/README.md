# Day 25: [Four-Dimensional Adventure](https://adventofcode.com/2018/day/25)

## Part 1

A simple brute-force approach worked well and very fast here.

The idea is that we create a list of constellations initially populated by one point each. Then we go over each point and see if they can be merged with any of the following. And we start over. If we go through a full iteration without merging anything, we stop.