# Day 17: [Conway Cubes](https://adventofcode.com/2020/day/17)

## Part 1

I used a HashSet for part 1, to store the active cubes, and I hard-coded the 26 neighbor positions.

## Part 2

For part 2, I had to stop hard-coding the neighbor positions and instead generate them with itertools multi_cartesian_product.

## Update

Optimized it by moving the call to multi_cartesian_product to a static section, to do it only once.