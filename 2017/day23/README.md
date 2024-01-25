# Day 23: [Coprocessor Conflagration](https://adventofcode.com/2017/day/23)

## Part 1

A simple reuse of [Day 18](../day18/README.md).

NB: With the reuse, it took me about 6-7 minutes to do this part after reading the description, without trying to go especially fast. To get into the [leaderboard](https://adventofcode.com/2017/leaderboard/day/23), 5 minutes was the limit.

## Part 2

Part 2 was way to slow to run with the Rust interpreter and you were supposed to manually analyse the assembly and find a way to optimize it. Unfortunately I'm not very good at it and didn't manage.

So instead I went for the approach of converting the assembly into C code, and running that. It's still slow, takes about 15 minutes to run, but that makes about 1000 times faster than the Rust interpreted version.