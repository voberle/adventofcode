# Day 14: [Docking Data](https://adventofcode.com/2020/day/14)

## Part 1

There were two main challenges in this part:

First one was how to apply the bitmask in the simplest way. My solution doesn't use an external crate and looks fairly small.

Second one was how to have a reference to the bitmask when applying the instructions, and avoid doing a copy of it. Here I use the first time [MaybeUninit](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html) and `unsafe`.

## Part 2

Second part took me way longer than it should have, because I got confused and assumed that it couldn't be brute-forced. But since there no more than 9 Xs in a masked address, it's not so many combinations and brute-forcing is all fine.

## Update

After reading through the [Reddit post on hard inputs](https://www.reddit.com/r/adventofcode/comments/kcybyr/2002_day_14_part_2_but_what_if_the_input_is_harder/), with more Xs, I saw what I had missed to make the optimized solution work.

I had the right solution for doing intersections of bitmasks, but the smart trick was to use that on each sub-sequence memory section to add the intersect with a *negative* value. That keeps it simple enough, and amazingly worked on the first try.
