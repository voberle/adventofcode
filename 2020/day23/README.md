# Day 23: [Crab Cups](https://adventofcode.com/2020/day/23)

## Part 1

I implemented first part with a vec and `rem_euclid`. It's a bit ugly but it works. It doesn't matter too much anyway considering what's coming with part 2...

## Part 2

Of course the naive implementation of part 1 doesn't work for part 2.

After brainstorming a bit on this one and drawing various ideas on paper, I found the solution: I could track the cups with a "linked list", but in a very efficient way by having a vector where the index is the cup and the value is the next cup.

That would require to have a vector with 1 million elements, but on each cup move, I would only have to modify a few of them (3 to be precise).

That way doing 10 million moves takes only 150 ms.
