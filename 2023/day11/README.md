# Day 11: [Cosmic Expansion](https://adventofcode.com/2023/day/11)

Another case where the first implementation of part 1 didn't scale at all for part 2 and had to be redone.

In the first attempt, the universe was modelled as a two-dimensional array, but with the universe growing 140 M wide, it had to change.
So I changed the model to only list the positions of the galaxy.

The nice thing is how I approached the refactoring of the expansion function:
I first replaced all iterators in it with index accesses of the 2-dimensions array, validating that it still works,
and that was then easier to convert to list of galaxies model.

Starting to use `BufRead` and traits so that the input parsing code can be used on stdin or on files (for tests).