# Day 19: [Go With The Flow](https://adventofcode.com/2018/day/19)

## Part 1

This was a follow-up to [Day 16](../day16/README.md), but I wasn't able to reuse that much code.

Anyway, part 1 was fairly simply once you understood the explanations.

## Part 2

The part 2 was the exercise of manually reverse-engineering the assembly and optimizing it. Normally I'm not very good at this, but this time I managed without using any external hint at all.

By observing the content of the registries, I saw there was a number that remained always there, small for part 1, big for part 2. I supposed it was the input.

So I started by understanding where in the assembly this number was calculated. The first line in the assembly is actually a jump to the code that calculates this, everything after line 17. That part was relatively easy to understand.

I then proceeded to understanding the main loop. After annotating the assembly, I translated it into Rust code. It was clear there were two loops, so it was just a matter at getting the logic right. I could verify that my code was correct by checking I got the correct part 1 output.

Finally, I understood that this code was doing the sums of all the factors of the input. This was then easy to write in an optimized way.

The [annotated_input](resources/annotated_input) file has my analysis of the assembly.