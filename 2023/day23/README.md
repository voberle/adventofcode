# Day 23: [A Long Walk](https://adventofcode.com/2023/day/23)

Got part 1 solved by implementing Dijkstra's algorithm, but it turns out that was a lucky hit, and using something called *shortest* path algorithm to find the longest path isn't the best idea. For part 2 it doesn't work at all and needs to be redone.

Two options:
* Pure brute force: Graph isn't that big so should work.
* Another algorigthm.

In either case would be good to convert the map into a proper graph, as this would avoid having to walk and count the steps all the time.