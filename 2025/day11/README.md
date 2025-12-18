# Day 11: [Reactor](https://adventofcode.com/2025/day/11)

## Part 1

Most work went into converting the input into a graph implemented only with Vec.

Then a DFS found all paths.

## Part 2

The number of paths to find in part 2 was very big, so some tricks were required.

I refactored the DFS code to be able to add caching to it.

Then with a Graphviz output, I saw in which order we pass the required nodes.
We see that all paths pass first by fft and then dac.
So we can count all paths from svr -> fft, fft -> dac and dac -> out and multiply the results.
