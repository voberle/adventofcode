# Day 20: [Pulse Propagation](https://adventofcode.com/2023/day/20)

Part 1 was fun to implement, with a clean use of a trait implemented by each module.
I had to be careful with the design to avoid running into problems with mutable configuration being borrowed again, but as I anticipated this early, I got it quickly right.
The initialization of Conjunction was a bit annoying to get right.
For the test inputs, they go back regularly to their initial state, so it's possible to optimize it even further, however that's not the case for the real input.

Part 2 was a very short description, inviting for trying a brute-force version that of course doesn't work.
It seems it would require analyzing the graph and finding some pattern in it (cf Day 8). Not feeling too much doing this for now.