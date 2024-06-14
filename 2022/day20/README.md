# Day 20: [Grove Positioning System](https://adventofcode.com/2022/day/20)

## Part 1

This part 1 sounded simple, but created me quite some trouble.

The test input doesn't contain any duplicate number, so I assumed it was the same for the real input, but the real input does. That left me confused for a while about why things worked on test but not with real input.

Then it was hard to get the move method that uses remove and insert working. In fact, I tried to do this one first, but as I didn't manage, I went for a simpler approach of using swap. Once swap version worked and I saw that insert/remove should work for part 2, I went back to it and got it working.

## Part 2

So my move method was still buggy, as it didn't work on the real data of part 2.

At the end, it was simply that my move method was way too complicated. The final solution was actually simple:

- Get the position of the item to move.
- Remove the item to move.
- Add the value to the position, modulo the length of the list (so 1 item less than the full list).
- Insert the element at the found position.

What got me into the rabbit hole is that initially, I didn't remove the item *before* doing the position calculations, which complicated everything.