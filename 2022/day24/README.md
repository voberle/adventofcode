# Day 24: [Blizzard Basin](https://adventofcode.com/2022/day/24)

## Part 1

I understood early that we don't need to track all blizzards on each minute. We can calculate for each blizzard where they will be at each minute. When doing the path finding, we are only interested in the state of 4 positions, the ones around our current position.

Also to find if a specific position is affected by a blizzard, we only need to check a sub-set of all blizzards, since only those on same row or column can reach it.

So I parsed the valley as just the blizzard positions, and saved them so that it's easy to find them by column or row.

Then the function to see if there is a blizzard at a position for a specific minute was fairly easy.

With that in hand, it was just a matter of applying Dijkstra algorithm, with the node being the position + the minute.

## Part 2

