# Day 16: [Flawed Frequency Transmission](https://adventofcode.com/2019/day/16)

## Part 1

Part 1 wasn't hard with a bit of experience with iterators. I like how I created an iterator for the pattern, which I then call while creating the signal.

## Part 2

Part 2 was tricky, but I found a way to optimize the brute-force version so that it runs in a few minutes:

The insight was that if we want the nth digit, we only need to calculate things for t [n..], can ignore all the digits before. Since our offsets are quite big, this helps a lot.

Also, if the offset is bigger than half the signal, then all pattern digits we need to use are 1. It simplifies things as there is no need to do complex pattern stuff.

That made it possible to brute-force it, as it run in a few minutes on the real input.