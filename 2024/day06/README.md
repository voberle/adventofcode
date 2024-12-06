# Day 6: [Guard Gallivant](https://adventofcode.com/2024/day/6)

## Part 1

As far as a map traversal goes, that is almost as easy as it gets. I kept my implementation simple, no smart trick: A grid with chars for the original map, and a grid of same size to mark the visited positions.

## Part 2

The approach I took for part 2 seemed straightforward enough.

A loop happens when we reach a previously visited place with the same direction.
So as we walk through the map, on each step we try to place an obstruction and check if we reach a loop.

This worked quickly for the test input, but not the real input. I had a few bugs...

First I didn't exclude putting an obstruction on the starting position.

Second, I was simply counting how many loops I hit, but some of those loops were caused by the same obstruction position.

Finally, the last bug took me a while. It turned out to be a problem when I was going on the same path back. I allowed to place obstructions on an already visited part, which cannot happen, since I couldn't have got to this place if there was an obstruction.

At the end, it's quite a nice solution, and fast, with both parts running in 15 ms.