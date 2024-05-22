# Day 24: [Arithmetic Logic Unit](https://adventofcode.com/2021/day/24)

## Part 1

This one was hard.

First it was quite confusing that the sample number in the text is not necessarily valid (it wasn't for me).

Once I had realized that a pure brute force version was clearly not possible, I analyzed the input. It was quickly clear there were 14 different parts that were very similar. I took one of these part and converted it to Rust code, first line by line, and the I optimized it into a shorter version. I also wrote code to extract the constants of the input (i.e. the differences between the parts).

However even with the Rust version of the input, it was still too slow to brute-force it.

I tried to manualy analyze what each part is doing and which numbers are possible, but I found it too complicated, so I decided to focus on solving it with code.

The key insight came when I realized that a lot of z are repeating themselves. There came the solution!

First I go through the 14 parts in order, and save all possible z's that can generated for each part. It's quite a lot (a few millions) but reasonable for a computer.

Then I start from the end, from last part to first part. For the last part, I check among the z's found previously which one can produce 0. Then with those z, I do the same with the before-last part, and so on. On each step, I save with digits that work.

Then finally I just need to take the biggest if the digits that work for each part.

## Part 2

Part 2 was then trivial.

My final code has lots of code not used, but which shows how I got there.

The solution runs in 10 seconds.