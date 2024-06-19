# Day 16: [Proboscidea Volcanium](https://adventofcode.com/2022/day/16)

## Part 1

I got stuck a while on this one. My initial approach with a recursive function didn't work, as I didn't manage to make it stop.

After leaving the problem aside for a few days, I went for an iterative method, calculating all possible states for each minute. With this approach, it's easier to handle duplicate states (just put everything in a map). State is:

- Current valve
- List of open valves

And we keep the maximum pressure we will reach for these states.

## Part 2

Unfortunately my part 1 approach didn't perform well for part 2. Storing two valves in the state made the number of states grow in the millions quickly.

Replacing the vector of boolean with a bitmask for the opened valves helped a bit, but not enough.

Finally, I found a way to prune a bit the list of states on each iteration, by counting the number of opened valves and only keeping the states with the maximum known opened valves count. This brings the runtime to around 15 seconds, not fast, but acceptable.

That pruning hackish, as it is a bit different for test and real data unfortunately.

## Update

I saw on Reddit that others also struggled to limit the search space.

One simple idea I saw there that also works is to to add a requirement that you must never pass by a valve without opening it if the flow rate is above a certain value.

Many also compressed the graph, removing the valves that cannot be opened. I had thought at it, but didn't see how much it would help versus the extra work. Could have been finding the shortest distance between all pairs of valves with positive flow rate (using Dijkstra), followed by the travelling salesman problem.

Also it's probably possible for part 2 to simulate both separately. Have one visit a certain number of valves, and the other visits the rest, and then find the best combinations of pairs. I'm not sure how I would have applied this to my approach tough.
