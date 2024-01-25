# Day 24: [Electromagnetic Moat](https://adventofcode.com/2017/day/24)

## Part 1

Part 1 solves itself with a recursive function exploring all the bridge possibilities.

Note that the intermediate data structure I created is probably overkill for the task. It got me to practice using lifetimes at least.

## Part 2

Maybe my part 1 wasn't the best, I'm not sure, but part 2 was somehow tricky to do with it.

I ended up simplifying the whole thing at the price of using more memory, by saving all possible bridges and the looking for the strongest and longest ones.