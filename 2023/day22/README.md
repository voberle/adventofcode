# Day 22: [Sand Slabs](https://adventofcode.com/2023/day/22)

## Part 1

To find out if two bricks overlap or not, I used a few tricks that simplified things quite a bit:

- Each brick had its coordinates ordered (p1 was always smaller than p2). This assumption allows for many simplifications later.
- Each brick had its direction identified, if it was a brick along the X, Y or Z axis.
- To figure out of two bricks overlap in the X-Y axis, I "flattened" them temporarily, setting their Z to 0.

Then I classified each brick in two HashMap, indicating at which height their bottom and top where.

Finally to figure out if a brick caused something to collapse, I brute forced it: I removed the brick, tryied collapsing things down and compared to the original. Was fast enough, around 15 seconds.

## Part 2

Part 2 turned out to work nicely with the same method.