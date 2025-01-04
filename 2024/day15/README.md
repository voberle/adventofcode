# Day 15: [Warehouse Woes](https://adventofcode.com/2024/day/15)

## Part 1

Lots of code was needed for parsing the input and handling everything, but only pushing the boxes was a bit harder. Drawing it on paper made it easy to understand what needed to be done however.

## Part 2

Extending my initial part 1 code to support the enlarged map was too ugly: It would be have been very difficult and likely very buggy.

So instead I took several refactoring steps to make the part 1 simpler and easy to extend to support part 2.

- I separated moving a block of boxes from finding a block of boxes.
- The moving code was then exactly the same for normal and enlarged maps. I could test it separately and didn't have to worry about anymore.
- For finding a block of boxes, I changed it into a model using recursion. That simplified the code a lot, but also made it relatively simple to add enlarged map support.

At the end, I find my solution quite nice:

- A function that does a robot move, with only one simple match, map size agnostic.
- A function that shifts blocks, using a fairly simple iterator, map size agnostic.
- A recursive function that finds blocks to move, with a simple special case to support going up/down on large maps.