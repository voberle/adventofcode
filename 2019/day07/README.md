# Day 7: [Amplification Circuit](https://adventofcode.com/2019/day/7)

## Part 1

Reusing the Intcode computer. Nothing complicated for part 1.

## Part 2

Part 2 required supporting interrupting the execution loop.

## Multi-threaded version

The code ran already very fast (1.7 ms for both parts), but the exercise seemed well suited to the use of channels, so I tried it.

It works, but it runs 10 times slower than the normal version.

## Previous days

To validate that the Intcode computer works on previous days as well, I put the tests and tasks of previous days in `previous_days` module.

Run with:

    cargo t --features previous_days

NB: Feature `previous_days` must be declared in `Cargo.toml`.
