# Day 21: [Scrambled Letters and Hash](https://adventofcode.com/2016/day/21)

## Part 1

Another exercise of converting a string based on instructions.

Got to use a few nice Vec functions: `swap`, `rotate_left`, `rotate_right`, `splice`.

## Part 2

The rotate position case was tricky in part 2. I solved it with some kind of reverse mapping table. Unfortunately it doesn't work for the test input, only the real one.