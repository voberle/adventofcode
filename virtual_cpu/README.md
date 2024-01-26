# Virtual CPU for Advent of Code instruction puzzles

This project aims to be able to run and solve the AoC "instructions" puzzles, where we get a set of assembly-style instructions and we need to execute them.

It is composed of a library with all the common code, and a main that uses the library to solve the various puzzles.

## Supported days

- 2017 - Day 23: [Coprocessor Conflagration](https://adventofcode.com/2017/day/23)
  - Part 1
- 2017 - Day 18: [Duet](https://adventofcode.com/2017/day/18)
  - Part 1
- 2016 - Day 25: [Clock Signal](https://adventofcode.com/2016/day/25)
  - Part 1
- 2016 - Day 23: [Safe Cracking](https://adventofcode.com/2016/day/23)
  - Part 1
- 2016 - Day 12: [Leonardo's Monorail](https://adventofcode.com/2016/day/12)
  - Part 1
  - Part 2

## Execution

Input files are in the `src/bin/input` directory (not in git). There is an `answers` file that lists the expected results, and for each day a file named `dayYYYY_DD_input` with the real input.

Run all the supported puzzles with:

    cargo r --bin main