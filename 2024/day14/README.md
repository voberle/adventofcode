# Day 14: [Restroom Redoubt](https://adventofcode.com/2024/day/14)

## Part 1

This part was quite trivial, with the help of `rem_euclid`.

## Part 2

To find the Easter egg, I assumed that the Christmas tree would fill the full picture, and therefore the top corner would have no robots.

Turns out this is not true. The text says **most** of the robots should arrange themselves into a picture of a Christmas tree, not all.

Based on this, I started searching for pictures with just a few robots in the corner. It didn't work until I switched from the left to the right top corner. Then at some point the tree appeared. It turns out that the tree was in a square a bit in the middle of the picture, but not exactly.

So for the actual code, I took the approach of looking for a cluster of 9 robots in a square.