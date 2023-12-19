# Day 19: [Aplenty](https://adventofcode.com/2023/day/19)

In part 1, a big part of the work was parsing the more complicated input and building the correct data structure.
I got part 1 done without using recursion.

Part 2 was significant work again, as the algorith of part 1 could not be reused.
I traversed all the graph using recursion this time. It took me some time to get the recursion method return value correct,
as initially I tried to have it return sub-counts or merged ranges. It turned out that simply returning the range for that path was the best.