# Day 11: [Dumbo Octopus](https://adventofcode.com/2021/day/11)

## Part 1

This one felt similar to [day 9](../day09/README.md), but one had to be careful that adjacent positions include the ones in diagonal here.

Besides this, implementing the 3 steps nicely one after each other worked well.

## Part 2

Part 2 felt it would require a very big number of steps and be an optimization problem, but not at all, so it was trivial.

## Update

Many other solutions used recursion for the flashing, which results in a bit shorter code.

One [nice short Rust solution](https://www.reddit.com/r/adventofcode/comments/rds32p/comment/ho76jny/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) also gave a much shorter version to find the adjacent positions, which I incorporated in my solution.