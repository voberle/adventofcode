# Day 12: [The N-Body Problem](https://adventofcode.com/2019/day/12)

## Part 1

Besides carefully reading the instructions, nothing hard there.

## Part 2

Part got for sure harder. I had the right intuition that we had to find the LCM (Least Common Multiple) for a sub-set of the coordinates. But initially I searched when pairs of moons were repeating, which somehow worked for the first example, so it got me in the wrong direction for a bit. But the right thing is to realize that all the moons depend on each other, but x, y and z are independant. So we have to find when the set of x, y and z repeat and take the LCM of those.
