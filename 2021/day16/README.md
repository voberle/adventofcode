# Day 16: [Packet Decoder](https://adventofcode.com/2021/day/16)

## Part 1

I did part 1 initially in a hacky way, just parsing all the packets and printing the version numbers, them getting the sum of it:

    awk '{s+=$1} END {print s}' version_numbers

I did this as I wasn't sure how to organize my code before knowing what part 2 was.

Then I did a cleaned up version.

## Part 2

