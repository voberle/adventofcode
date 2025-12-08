# Day 8: [Playground](https://adventofcode.com/2025/day/8)

## Part 1

There are 3 steps in my implementation.

First I create all possible connections and calculate their distance. There are 1 million of them, but it's still very fast (25 ms).

Then I connect the requested number of circuits. For this, I used a vector where the index is the same as in the positions list, and the value identifies which circuits we are part of. I go through the ordered list of connections. For each connection, I check that the boxes are not part of the same circuit already (they have different circuit ID aka values in the circuits vector) and I connect them by setting them to the same circuit ID.

Finally, I calculate the size of all circuits and get the 3 biggest ones.

## Part 2

I didn't expect it, but part 2 was easy.