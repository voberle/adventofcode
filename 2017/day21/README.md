# Day 21: [Fractal Art](https://adventofcode.com/2017/day/21)

## Part 1

This one turned out to be hard because I misunderstood what rotating means in the instructions. Initially I rotated by 1 element, instead of rotating the square by 90 degrees. This took me a while to figure out.

Besides this, I manage the while implementation by having the squares stored in `Vec<bool>`, which turned out to work better than I expected.

## Part 2

Trivial with part 1.