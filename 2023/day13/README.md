# Day 13: [Point of Incidence](https://adventofcode.com/2023/day/13)

## Part 1

I implemented a brute force approach but relatively optimized, and it runs nicely fast.

I used a `Vec<char>` for the two-dimensional array this time. This is great for getting the lines, but the columns require a copy and it's far less optimal.

## Part 2

Part 2 still worked with the same approach, of trying all possible smudge changes, it remained fast enough.
Main challenge here was understanding the task correctly, that the original reflection could still be valid and simply needed to be ignored.

## Update

Updated to match other years better.