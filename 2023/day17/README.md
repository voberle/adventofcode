# Day 17: [Clumsy Crucible](https://adventofcode.com/2023/day/17)

## Part 1

Here we had an implementation of Dijkstra shortest path algorithm, but with special moving rules. It was a really challenging one for me.

First I had not implemented Dijkstra shortest path before, so I had to learn it. While it's not very hard, one has to be precise in the implementation for it to work.

But of course the main challenge was dealing with the custom coordinates system. I had the right intuition that something like this was needed, but it took me multiple attempts to get the design right. My earlier attempts were both too complicated and/or incorrect. As often in retrospect, it feels obvious.

## Part 2

Part 2 required to change the condition on straight lines. Only tricky part was the condition that it cannot stop at the end if has not moved a min of 4 blocks straight.

## Optimization

As I'm using 3 HashMap/HashSet in the algorithm, I tried out switching them to [FxHash](https://github.com/cbreeden/fxhash).

It sped things up over 2x (60ms vs 130ms for part 1).