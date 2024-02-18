# Day 21: [Chronal Conversion](https://adventofcode.com/2018/day/21)

## Part 1

This puzzle is a follow-up to [Day 19](../day19/README.md), with the same set of instructions and a different puzzle.

The description is a bit tricky to understand, and there are some parts that seem just there to send you to a wrong bath (the bani check at the beginning).

Since now I'm better at analysing the assembly, I started doing that, [annotating it](resources/annotated_input) like in day 19.

As I could identify when the program exits, I thought I could guess the values that cause it to do so by looking at it, but that turned out complicated. So instead I identified one part of the code that was looping heavily, lines 18-25, and figured out it was simply doing a division.

I added this optimization to the code:

    if ip == 18 {
        regs[5] = regs[4] / 256;
        ip = 26;
    }

which made it run fast enough so that the rest could be brute forced.

I added code to "kill" the program after a certain number of executed instructions and with a few attempts I found the r0 that causes the program to stop with the smallest number of instructions.

## Part 2

