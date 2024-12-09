# Day 9: [Disk Fragmenter](https://adventofcode.com/2024/day/9)

## Part 1

It was a bit of work, but not very hard. I created the block list and moved them (using `Vec::swap`).

## Part 2

For part 2, I decided to track groups instead of individual spaces/files in my blocks. I moved all the part 1 implementation into a separate module and rewrote it for part 2.

The move method turned out a bit ugly and complicated, and because of moving things around in a vector, the whole implementation is rather slow (70 seconds).