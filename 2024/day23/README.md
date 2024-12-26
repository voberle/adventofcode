# Day 23: [LAN Party](https://adventofcode.com/2024/day/23)

## Part 1

I doubt my approach is the most efficient, but it did the job.

First I built a graph of the connections, i.e. a map with each computer to its connections.

Then for each computer, I go through each pair of its connections and check if they are connected. If they are, the computer and the pair from a group of 3.

Finally I filter for computers start with 't'.

It takes 9 ms, which feels a bit slow, so likely there is a better way.

## Part 2

To find the biggest group of all connected nodes, I took the approach of starting with the groups of 3 and building all groups of 4 out of them, then all groups of 5 and so on until I have only one group left.

At first it didn't seem to work as it was way too slow. But with a bunch of optimizations, I got the answer.

One set of optimizations was to implement the graph with a grid indicating if two nodes are connected, making it very fast to check for a connection.

Then I improved the new group building until it was fast enough.

Now it runs in 18 seconds.