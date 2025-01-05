# Day 21: [Keypad Conundrum](https://adventofcode.com/2024/day/21)

## Part 1

Here is one of those problems that require some real thinking to figure out the design.

I wrote out a model for the keypads, which mainly give the paths to go from one key to another.

Then I build all possible directions phase by phase.

It's not very fast however: 7 seconds.

## Part 2

My initial approach was to optimize part 1 code. That included reducing the number of options we explore, something I ended up doing even too aggressively (those reductions worked for part 1 but were too aggressive for part 2).

Anyway, none of the optimizations helped with the problem that the number of directions was many billions as we approached the 25th robot.

I was quite sure memoization was the way to go, but it took me a while to figure out how to use it. Finally I figured out it could be applied on a list of directions + the remaining number of robots to run.

When I got that working for part 1, and that part 2 was giving me an answer quickly (even if wrong), I knew I was on the right track. I just had to revert some of the optimizations I did to get the right answer.

And it's fast now: Both parts in 4 ms.
