# Day 23: [LAN Party](https://adventofcode.com/2024/day/23)

## Part 1

I doubt my approach is the most efficient, but it did the job.

First I built a graph of the connections, i.e. a map with each computer to its connections.

Then for each computer, I go through each pair of its connections and check if they are connected. If they are, the computer and the pair from a group of 3.

Finally I filter for computers start with 't'.

## Part 2

