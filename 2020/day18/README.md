# Day 18: [Operation Order](https://adventofcode.com/2020/day/18)

## Part 1

At first I attempted an implementation using recursion, but it ended up too complicated and didn't work.

So I looked up how parsing of mathematical expressions is typically done, and found that one common method is [Shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm). I implemented [this pseudo-code](https://www.geeksforgeeks.org/expression-evaluation/) and it worked nicely.

## Part 2

With that implementation, part 2 was super easy, as I just needed to adjust the precedence function.