# Day 18: [Lavaduct Lagoon](https://adventofcode.com/2023/day/18)

This exercise was all about finding the surface of a polygon.

## Part 1

After the silly attempts that worked only on simple polynoms, I implemented flood-filling approach.
This got me the part 1 answer.

## Part 2

The polygon became huge and flood-filling wasn't possible anymore.
So I implemented the Shoelace algorithm, which fortunately didn't end-up being too difficult with some clear description I found online.
