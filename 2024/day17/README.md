# Day 17: [Chronospatial Computer](https://adventofcode.com/2024/day/17)

## Part 1

This is a classic of AoC, developing a processor simulator. This one had unusual instructions.

I probably wrote two much code, could have dealt with the integers directly instead of converting it all to enums.

## Part 2

The second part is also the classical reverse engineering of the program.

First I converted the program into Rust, which helped understand it:

    while a != 0 {
        b = a % 8;
        b ^= 3;
        c = a / 2u32.pow(b);
        b ^= c;
        b ^= 3;
        a /= 8;
        output.push(b % 8);
    }

To quit the loop, A must be zero. B and C are working registers, they reset on each iteration. At the end of the iteration, last 3 bits of B are output.

We see that on each iteration, A is divided by 8, meaning it's shifted 3 bits right.

So the idea is to rebuild A by looking at which B we need to produce, starting with the last one.

- What is the smallest A that produces a B that is 0 (the last number of the program)?
- Then shift this A 3 bits to the left, and use it as starting point to find the smallest A that produces a B equal to 3.
- And so on.

For some reason I don't understand, the final A didn't work: It generated an output where the second number was 3 instead of 4.

However I knew this value of A was too low, and that I was close. So I just tried all values above A until I found the one that produced the correct output.
