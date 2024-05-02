# Day 9: [Smoke Basin](https://adventofcode.com/2021/day/9)

## Part 1

I just reused my Grid code, so was easy.

## Part 2

For part 2, I realized that there is a basin for each low point, and that basins are bordered by 9s.

With that, I implemented something a bit similar to Dijkstra to explore all positions around each low points. It makes for a readable and clean solution.