# Day 5: [Hydrothermal Venture](https://adventofcode.com/2021/day/5)

## Part 1

Simply brute-forced it by putting all possible points in a hashmap.

## Part 2

In part 2, I had to rework how generation of the line points work, to handle correctly diagonal lines "going up", i.e. with x1 > x2 and y1 < y2. In the process, I removed the distinction between horizontal, vertical and diagonal lines.