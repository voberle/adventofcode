# Day 13: [Packet Scanners](https://adventofcode.com/2017/day/13)

## Part 1

I didn't want to loop and track the scanner position here, but have a formula that would give me the scanner position for any range at any time.

The idea came from looking on [OEIS](https://oeis.org/A271751): For each range I'm interested in (there aren't that many possibilities) I create a little vector with the range * 2 first values, and next ones can be easily obtained by doing a time % range.

Once I have this way of calculating the scanner position, calculating the severerity is a simple loop on the ranges and checking if the scanner position is 0.

## Part 2

Brute-forcing it works well.

Note that we couldn't reuse the severity function from part 1, as the severity for the layer 0 was always 0.