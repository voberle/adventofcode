# Day 21: [Allergen Assessment](https://adventofcode.com/2020/day/21)

## Part 1

With the list of foods that contain each allergen, we take the intersection of each. This gives us a small set of ingredient candidates for each allergen.

Then it's candidate pruning like in [day 16](../day16/README.md).

## Part 2

I did most of the work in part 1, so part 2 was easy.

My implementation does clone lots of strings, but input is small so no big deal (runs in 2 ms).

## Update

Inspired by a Reddit solution, I replaced most string copying by passing around &str instead. It works with a few lifetime annotations. I also had to remove the use of regex for the parsing, as that was causing trouble with the lifetimes.
