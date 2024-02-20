# Day 24: [Immune System Simulator 20XX](https://adventofcode.com/2018/day/24)

## Part 1

First this was quite some work to parse the input. I used the opportunity to try regex named capture groups.

Following carefully the instructions was important, and this got me nicely the test input working. However I struggled for a while to get it working with the real input, finally I got it by fiddling with the target filtering criteria. I needed a filter for damage is positive, but NOT one for damage being bigger than hit points.

And of course it had to be implemented while keeping the borrow checker happy.

## Part 2

