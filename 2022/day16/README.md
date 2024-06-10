# Day 16: [Proboscidea Volcanium](https://adventofcode.com/2022/day/16)

## Part 1

I got stuck a while on this one. My initial approach with a recursive function didn't work, as I didn't manage to make it stop.

After leaving the problem aside for a few days, I went for an iterative method, calculating all possible states for each minute. With this approach, it's easier to handle duplicate states (just put everything in a map). State is:

- Current valve
- List of open valves

And we keep the maximum pressure we will reach for these states.

## Part 2

