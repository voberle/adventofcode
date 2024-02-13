# Day 17: [Reservoir Research](https://adventofcode.com/2018/day/17)

## Part 1

That one was fun to do. At first, it wasn't clear to me how to proceed, but then by implementing the 3 parts of the water flow carefully, it worked out well and I got the water flowing correctly on first try.

Main issue for getting the right answer was counting correctly the tiles, as I assumed wrongly didn't have to filter by y.

My solution was initially a bit slow, at 8 seconds. I later optimized it to 1.3 seconds.

The visualization is very satisfying.

## Part 2

Trivial.

## Other approaches

It's interesting that I didn't think at all to use recursion, like many other people. It seems it gives shorter solutions, but not really that much simpler to understand. Some are much faster however. So I remain very happy with my approach.

Quoting a Reddit user:

> The fun part about solving this type of puzzle recursively is how the answer springs out so suddenly. Once I put the last piece into place, I looked at the output (a rendering of the map) thinking, "what? I'm done already?"
