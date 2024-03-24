# Day 23: [Category Six](https://adventofcode.com/2019/day/23)

## Part 1

The description makes you feel you should put each Intcode computer in its own thread, but I just ran them in a loop, passing -1 to as input before executing the code to "stimulate" it.

## Part 2

Part 2 wasn't hard, a simple extension of part 1.

## Multi-threaded version?

It would be a good exercise to convert this to using threads or even better futures. For part 1, it should be simple, but I'm not sure how easy part 2 would be.

I'm also still wondering if my Intcode exec() function should return a status (say `Output(i64), WaitingForInput, Halted`) like many others have done, or not?