# Day 13: [Knights of the Dinner Table](https://adventofcode.com/2015/day/13)

## Part 1

As the input wasn't too big, it could easily be brute forced. Found all possible seating arrangements with itertools permutation function, calculate the happiness change for each and picked the smallest.

Interesting is the nice `wrapping_index` function for getting the last element of an array with index -1, and vice-versa.

## Part 2

