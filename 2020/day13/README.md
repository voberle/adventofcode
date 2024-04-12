# Day 13: [Shuttle Search](https://adventofcode.com/2020/day/13)

## Part 1

Simple part 1, once you have the calculation for the next bus departure.

## Part 2

Part 2 was complicated.

While I was quite sure it wasn't going to be fast enough, I still implemented a brute force version. It worked fine for tests, but looked too slow for the real input (once I knew the answer, I estimated that the brute force version was going to take around 30 minutes).

The intuition for a faster solution was to calculate an LCM (Least Common Multiple) with offsets. I had no idea how to do this, but Google gave me [one implementation in Python](https://math.stackexchange.com/questions/2218763/how-to-find-lcm-of-two-numbers-when-one-starts-with-an-offset).

After some experimentations, I managed to make it work for more than 2 numbers and got the solution.

For the conversion, one trap was to use `rem_euclid` for the Python `%`, as we are dealing with negative numbers.