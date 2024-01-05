# Day 21: [RPG Simulator 20XX](https://adventofcode.com/2015/day/21)

## Part 1

The main task here was to create the list of the options the player had in the shop. I created list of options for each 3 item categories, and then found all possibilities with itertools multi_cartesian_product(). Merging them back into one each and ordering them allows to go through them by descending cost and pick the cheapest.

I started by creating all the shop items as `const`, but this was rather annoying to use as I couldn't use `String` for the names, and therefore it was hard to create dynamically the merged items with a new name (not necessary for the task but nice for debugging).

## Part 2

