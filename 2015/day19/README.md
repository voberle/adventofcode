# Day 19: [Medicine for Rudolph](https://adventofcode.com/2015/day/19)

## Part 1

Nothing special, brute-forced by adding all possible results in a HashSet.

## Part 2

This one was crazy. I tried brute-forcing it, but they were way too many possibilities.

How I got closer was trying from the end molecule to go back to the initial one. Doing this with a recursive approach turned out to work *sometimes only*, depending on the order of the input data. But when it did work, it produced always the same response, the correct one.

So the ugly solution is to try multiple times with random data.

Apparently they were patterns in the input data that could be used. I didn't find the ones that allowed for optimizations.