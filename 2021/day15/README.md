# Day 15: [Chiton](https://adventofcode.com/2021/day/15)

## Part 1

Part 1 was basically using my Grid and Dijkstra code almost unmodified, and it just works. Runs in 2.5 ms.

## Part 2

For part 2, I created a Cave trait that the shortest path code depends on, and which is implemented by Grid (for part 1) and by a BigCave for part 2.

It turned out that the only real difference between both implementations is the `get_risk_level()` method, which wasn't even very complicated (it helped that we had the big cave for the test input to validate it).

Runs in 30 ms for both parts.