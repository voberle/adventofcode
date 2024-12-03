# Day 3: [Mull It Over](https://adventofcode.com/2024/day/3)

## Part 1

This turned out very easy with regex. Nicest was to use for the first time `captures_iter` for a nicely short elegant solution.

## Part 2

Is there a way to do that part with only one regex?

Also I started using `LazyLock` for the initialization of the regex, which is easy and doesn't require another crate.