# Day 23: [Amphipod](https://adventofcode.com/2021/day/23)

## Part 1

The approach I took was to encode all possible positions as a short vector. Then for each vector, I have long and not so smart code that finds the next possible vectors that we can get from there. Finally, I apply Dijkstra to find the shortest path to the final state.

While the next positions code is long and ugly, the final solution works nicely and is fast (600 ms).

## Part 2

Fortunately the choice I made for part 1 were not too hard to extend to support the unfolded burrow.

For part 2, I had to replace the array with a vector for the burrow, which means we had to clone it instead of copying it. To compensate, I made the Dijkstra a bit faster by storing only the hashes in the set/map. Possibly encoding the whole burrow as an integer might speed things up more.

Both parts run in 2.7 seconds.

The next_positions code looks ugly, but it implements all the moving rules and in reality isn't that bad, as it gets the job done.