# Day 22: [Monkey Map](https://adventofcode.com/2022/day/22)

## Part 1

The non-rectangular map required a bit of thinking for the parsing and map implementation. I settled with a solution where the whole map is surrounded with a "void" tile. It makes the parsing fairly easy and especially it allows to avoid fully any boundary checking via indexes.

The code is a bit verbose, with lots of enums.

## Part 2

