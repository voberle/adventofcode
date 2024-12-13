# Day 13: [Claw Contraption](https://adventofcode.com/2024/day/13)

## Part 1

At first I considered complicated approaches, but then realized that with the limit of maximum 100 presses per buttons, it could be easily brute-forced.

So that's what I did. Runs in 5 ms.

## Part 2

The problem can be transformed into two linear equations, that have only one solution.
There is no concept of minimum number of tokens: There is either one solution or none.

    a * ax + b * bx = px
    a * ay + b * by = py

To solve them, I went into the rabbit hole of linear Diophantine equation, but that turned out to be complex stuff, and finally not necessary.

Instead, by using the elimination method, I got the value of b as a simple division. To check if the solution was an integer, it was just as simple as checking the modulo of the division.
